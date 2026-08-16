//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1113/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1113<F: Float>(t4234: F, t940: F, t238: F, t242: F, t10911: F, t343: F, t10928: F, t10930: F, t10935: F, t10939: F, t10942: F, t7037: F, t7089: F, t9113: F, t9116: F, t9159: F) -> (F, F, F, F, F) {
    let t10944 = t940 * t4234;
    let t10946 = t238 * t242 * t10944;
    let t10948 = t343 * t10911;
    let t10950 = t238 * t242 * t10948;
    let t10952 = F::cast_from(0.15358125e0_f64) * t10928 + F::cast_from(0.3071625e0_f64) * t10930 - t7089 + F::cast_from(0.27385555555555555556e0_f64) * t7037 + F::cast_from(0.5477111111111111111e0_f64) * t9159 - t9113 - t9116 - F::cast_from(0.16431333333333333333e0_f64) * t10935 + F::cast_from(0.49294e0_f64) * t10939 - F::cast_from(0.16431333333333333333e0_f64) * t10942 + F::cast_from(0.24647e0_f64) * t10946 + F::cast_from(0.24647e0_f64) * t10950;
    (t10944, t10946, t10948, t10950, t10952)
}
