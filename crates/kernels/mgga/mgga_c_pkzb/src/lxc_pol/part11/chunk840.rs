//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 840/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk840<F: Float>(t1769: F, t3453: F, t1733: F, t2592: F, t2645: F, t5279: F, t5289: F, t612: F, t6896: F, t6935: F, t8946: F, t8950: F, t8955: F, t8959: F, t8964: F, t8968: F, t8973: F, t8978: F, t8983: F, t8988: F) -> (F, F) {
    let t8991 = t1769 * t3453;
    let t8993 = -F::cast_from(0.80031500487063509015e-2_f64) * t8946 + F::cast_from(0.42874018118069736972e-3_f64) * t2592 * t8950 - F::cast_from(0.12862205435420921092e-2_f64) * t6896 * t8955 + F::cast_from(0.12862205435420921092e-2_f64) * t2592 * t8959 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t8964 - F::cast_from(0.42874018118069736972e-3_f64) * t2645 * t8968 - F::cast_from(0.21437009059034868486e-3_f64) * t2645 * t8973 - F::cast_from(0.42874018118069736972e-2_f64) * t5279 * t8978 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t8983 + F::cast_from(0.80031500487063509015e-2_f64) * t6935 - t5289 - F::cast_from(0.85748036236139473944e-3_f64) * t612 * t8988 - F::cast_from(0.20007875121765877254e-1_f64) * t8991;
    (t8991, t8993)
}
