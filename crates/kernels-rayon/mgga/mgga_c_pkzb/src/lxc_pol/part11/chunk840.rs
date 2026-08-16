//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 840/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk840(t1769: f64, t3453: f64, t1733: f64, t2592: f64, t2645: f64, t5279: f64, t5289: f64, t612: f64, t6896: f64, t6935: f64, t8946: f64, t8950: f64, t8955: f64, t8959: f64, t8964: f64, t8968: f64, t8973: f64, t8978: f64, t8983: f64, t8988: f64) -> (f64, f64) {
    let t8991 = t1769 * t3453;
    let t8993 = -0.80031500487063509015e-2_f64 * t8946 + 0.42874018118069736972e-3_f64 * t2592 * t8950 - 0.12862205435420921092e-2_f64 * t6896 * t8955 + 0.12862205435420921092e-2_f64 * t2592 * t8959 + 0.85748036236139473944e-3_f64 * t1733 * t8964 - 0.42874018118069736972e-3_f64 * t2645 * t8968 - 0.21437009059034868486e-3_f64 * t2645 * t8973 - 0.42874018118069736972e-2_f64 * t5279 * t8978 + 0.85748036236139473944e-3_f64 * t1733 * t8983 + 0.80031500487063509015e-2_f64 * t6935 - t5289 - 0.85748036236139473944e-3_f64 * t612 * t8988 - 0.20007875121765877254e-1_f64 * t8991;
    (t8991, t8993)
}
