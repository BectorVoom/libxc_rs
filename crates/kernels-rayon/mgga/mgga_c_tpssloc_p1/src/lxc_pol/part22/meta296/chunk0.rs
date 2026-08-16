//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1458/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1458(t13847: f64, t2990: f64, t2986: f64, t2987: f64, t4540: f64, t2989: f64, t3966: f64, t2960: f64, t4506: f64, t10224: f64, t1592: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13848 = t13847 * t2990;
    let t13850 = 0.18518518518518518518e-3_f64 * t2986 * t13848;
    let t13851 = t2987 * t4540;
    let t13861 = t2989 * t3966;
    let t13893 = 0.49382716049382716048e-3_f64 * t2960 * t4506;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    (t13850, t13851, t13861, t13893, t13895, t13896)
}
