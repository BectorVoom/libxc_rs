//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1149/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1149(t10120: f64, t774: f64, t1232: f64, t1625: f64, t3275: f64, t3272: f64, t1639: f64, t3260: f64, t1206: f64, t3342: f64, t4480: f64, t4397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12816 = t10120 * t774;
    let t12817 = t1625 * t1232;
    let t12818 = t12817 * t3275;
    let t12819 = t12816 * t12818;
    let t12822 = t3272 * t774;
    let t12823 = t1639 * t1232;
    let t12825 = t12822 * t12823 * t3275;
    let t12828 = t1639 * t3260;
    let t12829 = t1232 * t1206;
    let t12831 = t12822 * t12828 * t12829;
    let t12835 = 35.0_f64 / 576.0_f64 * t3342 * t4480;
    let t12836 = t4397 * t1206;
    (t12819, t12823, t12825, t12828, t12831, t12835, t12836)
}
