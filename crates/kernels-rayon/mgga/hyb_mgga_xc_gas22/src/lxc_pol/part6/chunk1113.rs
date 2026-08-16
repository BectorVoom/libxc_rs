//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1113/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1113(t4234: f64, t940: f64, t238: f64, t242: f64, t10911: f64, t343: f64, t10928: f64, t10930: f64, t10935: f64, t10939: f64, t10942: f64, t7037: f64, t7089: f64, t9113: f64, t9116: f64, t9159: f64) -> (f64, f64, f64, f64, f64) {
    let t10944 = t940 * t4234;
    let t10946 = t238 * t242 * t10944;
    let t10948 = t343 * t10911;
    let t10950 = t238 * t242 * t10948;
    let t10952 = 0.15358125e0_f64 * t10928 + 0.3071625e0_f64 * t10930 - t7089 + 0.27385555555555555556e0_f64 * t7037 + 0.5477111111111111111e0_f64 * t9159 - t9113 - t9116 - 0.16431333333333333333e0_f64 * t10935 + 0.49294e0_f64 * t10939 - 0.16431333333333333333e0_f64 * t10942 + 0.24647e0_f64 * t10946 + 0.24647e0_f64 * t10950;
    (t10944, t10946, t10948, t10950, t10952)
}
