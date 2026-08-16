//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 931/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk931(t31169: f64, t5234: f64, t114011: f64, t32721: f64, t1824: f64, t22705: f64, t22852: f64, t550: f64, t59: f64, t1831: f64, t31176: f64, t22804: f64, t32711: f64) -> (f64, f64, f64, f64, f64) {
    let t120341 = t5234 * t31169;
    let t120350 = t114011 * t32721;
    let t120363 = t22852 * t22705 * t59 * t1824 * t550;
    let t120375 = t31176 * t1831;
    let t120383 = t22804 * t32711;
    (t120341, t120350, t120363, t120375, t120383)
}
