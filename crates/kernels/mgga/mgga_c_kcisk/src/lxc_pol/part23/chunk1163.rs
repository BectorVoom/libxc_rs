//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1163/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1163<F: Float>(t2737: F, t32388: F, t9529: F, t9532: F, t3913: F, t532: F, t20: F, t2734: F, t2740: F, t32002: F, t32006: F, t32339: F, t32343: F, t32346: F, t32350: F, t32354: F, t32359: F, t32363: F, t32366: F, t32371: F, t32377: F, t32380: F, t32385: F, t9516: F, t9536: F, t9539: F) -> (F, F, F, F, F, F) {
    let t32390 = 0.11574074074074074074e-2 * t2737 * t32388;
    let t32391 = t9529 * t9532;
    let t32393 = t532 * t3913;
    let t32394 = t32393 * t20;
    let t32395 = t2734 * t32394;
    let t32398 = 0.92592592592592592593e-2 * t32339 * t9539 - 0.11574074074074074074e-2 * t32343 - 0.17361111111111111111e-2 * t9536 * t32346 - 0.23148148148148148148e-2 * t9536 * t32350 - 0.34722222222222222222e-2 * t32354 * t9539 + 0.27777777777777777778e-1 * t32359 * t2740 - 0.52083333333333333333e-2 * t32363 * t2740 - 0.10416666666666666667e-1 * t32366 * t2740 - 0.52083333333333333333e-2 * t32371 * t2740 - 0.61905925925925925925e-2 * t32002 - 0.23214722222222222222e-2 * t32006 - 0.116403125e-2 * t32377 * t32380 + 0.20104166666666666667e-2 * t9516 * t32385 + t32390 + 0.92592592592592592593e-2 * t32391 - 0.50925925925925925926e-1 * t32395 * t2740;
    (t32390, t32391, t32393, t32394, t32395, t32398)
}
