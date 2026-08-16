//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1310/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1310(t31033: f64, t7685: f64, t1983: f64, t33136: f64, t6996: f64, t652: f64, t6862: f64, t7467: f64, t2314: f64, t32670: f64, t32782: f64, t6999: f64) -> (f64, f64, f64, f64, f64) {
    let t119862 = t7685 * t31033;
    let t119867 = 2.0_f64 * t1983 * t6996 * t33136;
    let t119869 = t652 * t6862 * t7467;
    let t119871 = t2314 * t32670;
    let t119874 = t1983 * t32782 * t6999;
    (t119862, t119867, t119869, t119871, t119874)
}
