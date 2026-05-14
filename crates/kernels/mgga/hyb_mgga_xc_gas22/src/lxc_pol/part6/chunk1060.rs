//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1060/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1060<F: Float>(t10948: F, t238: F, t242: F, t10928: F, t10930: F, t10935: F, t10939: F, t10942: F, t10946: F, t7037: F, t7089: F, t9113: F, t9116: F, t9159: F, t10926: F, t968: F) -> (F, F, F) {
    let t10950 = t238 * t242 * t10948;
    let t10952 = 0.15358125e0 * t10928 + 0.3071625e0 * t10930 - t7089 + 0.27385555555555555556e0 * t7037 + 0.5477111111111111111e0 * t9159 - t9113 - t9116 - 0.16431333333333333333e0 * t10935 + 0.49294e0 * t10939 - 0.16431333333333333333e0 * t10942 + 0.24647e0 * t10946 + 0.24647e0 * t10950;
    let t10953 = t10926 + t10952;
    let t10954 = t10953 * t968;
    (t10950, t10953, t10954)
}
