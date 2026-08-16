//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 742/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk742(t3827: f64, t69: f64, t3844: f64, t608: f64, t1941: f64, t612: f64, t3847: f64, t3849: f64, t3851: f64, t3853: f64, t3855: f64, t3857: f64, t3859: f64, t3861: f64, t3863: f64, t3865: f64, t51: f64, t565: f64) -> (f64, f64, f64, f64, f64) {
    let t3867 = t69 * t3827;
    let t3869 = t608 * t3844;
    let t3871 = t1941 * t3827;
    let t3873 = t612 * t3844;
    let t3875 = t51 * t3827 / 6.0_f64 - t565 * t3844 / 18.0_f64 - t3847 / 48.0_f64 + t3849 / 240.0_f64 + t3851 / 640.0_f64 - t3853 / 4480.0_f64 - t3855 / 11520.0_f64 + t3857 / 103680.0_f64 + t3859 / 258048.0_f64 - t3861 / 2838528.0_f64 - t3863 / 6881280.0_f64 + t3865 / 89456640.0_f64 + t3867 / 0.21233664e9_f64 - t3869 / 0.31850496e10_f64 - t3871 / 0.74317824e10_f64 + t3873 / 0.1263403008e12_f64;
    (t3867, t3869, t3871, t3873, t3875)
}
