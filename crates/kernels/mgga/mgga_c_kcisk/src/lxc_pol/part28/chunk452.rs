//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 452/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk452<F: Float>(t2933: F, t2935: F, t68: F, t69: F, t142: F, t862: F, t85: F, t861: F, t73: F, t879: F, t880: F, t20: F, t71: F, t74: F, t79: F, t2863: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2936 = t2933 * t2935;
    let t2942 = t68 * t69;
    let t2943 = t142 * t862;
    let t2947 = t861 * t85;
    let t2948 = 1.0 / t2947;
    let t2949 = t73 * t2948;
    let t2950 = t879 * t879;
    let t2951 = t2950 * t880;
    let t2957 = 1.0 / t74 / t71 * t79 * t20;
    let t2958 = t2957 * t2863;
    (t2936, t2942, t2943, t2948, t2949, t2950, t2951, t2957, t2958)
}
