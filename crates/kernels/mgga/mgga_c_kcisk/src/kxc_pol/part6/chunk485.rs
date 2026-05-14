//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 485/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk485<F: Float>(t4636: F, t1964: F, t760: F, t755: F, t4722: F, t763: F, t4761: F, t591: F, t4787: F, t791: F, t1992: F, t794: F, t772: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5360 = 0.22831111111111111111e-1 * t4636;
    let t5371 = t1964 * t760;
    let t5372 = 1.0 / t5371;
    let t5373 = t755 * t5372;
    let t5380 = 0.68863333333333333333e0 * t4636;
    let t5387 = 0.17365833333333333333e0 * t4722;
    let t5396 = t1964 * t1964;
    let t5397 = 1.0 / t5396;
    let t5398 = t755 * t5397;
    let t5399 = t763 * t763;
    let t5400 = 1.0 / t5399;
    let t5408 = t591 * t4761;
    let t5415 = t591 * t4787;
    let t5438 = t791 * t791;
    let t5439 = 1.0 / t5438;
    let t5444 = 1.0 / t1992 / t794;
    let t5445 = t772 * t5444;
    (t5360, t5372, t5373, t5380, t5387, t5396, t5397, t5398, t5399, t5400, t5408, t5415, t5438, t5439, t5444, t5445)
}
