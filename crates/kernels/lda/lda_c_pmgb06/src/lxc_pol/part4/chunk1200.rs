//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1200/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1200<F: Float>(t11796: F, t11798: F, t15474: F, t15475: F, t15476: F, t15480: F, t15482: F, t15484: F, t15487: F, t15490: F, t15493: F, t15496: F, t15498: F, t15501: F, t15506: F, t11810: F, t11813: F, t15509: F, t15510: F, t15511: F, t15516: F, t15518: F, t15520: F, t15522: F, t15524: F, t15526: F, t15527: F, t15699: F, t9338: F, t9340: F) -> (F, F) {
    let t18168 = -t15474 - t15475 - t15476 - t15480 + t15482 + 0.19947266666666666 * t11796 + 0.13298177777777778 * t11798 - t15484 - t15487 - t15490 - t15493 - t15496 - t15498 - t15501 - t15506;
    let t18173 = 0.003030876351851852 * t11810 - 0.027012345679012346 * t11813 - t15509 + t15510 + t15511 - t15516 + t15518 - t15520 - t15522 + t15524 - t15526 + t15527 - t15699 + 0.033245444444444446 * t9338 + 0.19947266666666666 * t9340;
    (t18168, t18173)
}
