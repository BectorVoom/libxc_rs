//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1273/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1273<F: Float>(t110925: F, t9365: F, t1128: F, t32589: F, t3376: F, t3417: F, t1123: F, t43674: F, t32661: F, t15698: F, t273: F, t3422: F, t397: F, t32592: F, t32669: F, t1101: F, t2697: F, t3391: F, t918: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110926 = t9365 * t110925;
    let t110930 = t32589 * t3376 * t1128 * t3417;
    let t110931 = t9365 * t110930;
    let t110934 = t32589 * t43674 * t1123;
    let t110935 = t32661 * t110934;
    let t110938 = t15698 * t1128;
    let t110940 = t397 * t273 * t110938 * t3422;
    let t110941 = t32661 * t110940;
    let t110943 = t32669 * t32592;
    let t110947 = t1101 * t3391 * t918 * t2697;
    (t110926, t110930, t110931, t110934, t110935, t110940, t110941, t110943, t110947)
}
