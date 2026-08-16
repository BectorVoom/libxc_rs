//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1986/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1986(t13030: f64, t225: f64, t13062: f64, t13378: f64, t193: f64, t2379: f64, t15823: f64, t15800: f64, t15808: f64, t15814: f64, t15831: f64, t15816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47585 = t13030 * t225;
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47645 = t193 * t2379;
    let t51925 = t15823 * t225;
    let t51928 = t15800 * t225;
    let t51937 = t15808 * t225;
    let t52386 = t15814 * t225;
    let t53658 = t15831 * t225;
    let t53703 = t15816 * t225;
    (t47585, t47609, t47618, t47645, t51925, t51928, t51937, t52386, t53658, t53703)
}
