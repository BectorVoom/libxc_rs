//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1062/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1062<F: Float>(t260: F, t4305: F, t10888: F, t10890: F, t10893: F, t10898: F, t10913: F, t10915: F, t10922: F, t10924: F, t6969: F, t7116: F, t9008: F, t9189: F, t10928: F, t10930: F, t10935: F, t10939: F, t10942: F, t10946: F, t10950: F, t7037: F, t7123: F, t9159: F, t9171: F, t9172: F) -> (F, F, F) {
    let t10979 = t260 * t4305;
    let t10992 = 0.19419375e1 * t10888 - 0.258925e1 * t10890 - 0.1294625e1 * t10893 + 0.258925e1 * t10915 - t7116 + 0.40256666666666666667e0 * t6969 + 0.80513333333333333333e0 * t9008 - t9189 - 0.301925e0 * t10898 + 0.905775e0 * t10913 - 0.412621875e-1 * t10922 + 0.16504875e0 * t10924;
    let t11002 = 0.82524375e-1 * t10928 + 0.16504875e0 * t10930 - t7123 + 0.27595e0 * t7037 + 0.5519e0 * t9159 - t9171 - t9172 - 0.16557e0 * t10935 + 0.49671e0 * t10939 - 0.16557e0 * t10942 + 0.248355e0 * t10946 + 0.248355e0 * t10950;
    (t10979, t10992, t11002)
}
