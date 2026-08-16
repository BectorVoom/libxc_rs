//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 495/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk495(t1572: f64, t1584: f64, t1591: f64, t1593: f64, t1595: f64, t1597: f64, t1599: f64, t1881: f64, t1886: f64, t1891: f64, t1896: f64, t1901: f64, t418: f64) -> f64 {
    let t1905 = 0.85748036236139473944e-3_f64 * t1572 - 0.85748036236139473944e-3_f64 * t1584 - 0.40015750243531754508e-2_f64 * t1591 + 0.40015750243531754508e-2_f64 * t1593 + 0.80031500487063509015e-2_f64 * t1595 - 7.0_f64 / 144.0_f64 * t1597 + 0.12862205435420921092e-2_f64 * t418 * t1881 + 0.42874018118069736972e-2_f64 * t418 * t1886 - 0.85748036236139473944e-3_f64 * t418 * t1891 + 0.42874018118069736972e-3_f64 * t418 * t1896 - 0.42874018118069736972e-3_f64 * t418 * t1901 + 0.20007875121765877254e-2_f64 * t1599;
    t1905
}
