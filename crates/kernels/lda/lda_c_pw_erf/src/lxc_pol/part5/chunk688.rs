//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 688/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk688<F: Float>(t4507: F, t558: F, t593: F, t6723: F, t352: F, t4515: F, t4522: F, t1996: F, t4479: F, t2010: F, t4475: F, t1325: F, t3965: F, t3974: F, t4488: F, t4506: F, t4948: F, t6690: F, t6693: F, t6697: F, t6700: F, t6703: F, t6706: F, t6708: F, t6713: F, t6717: F, t6720: F, t6725: F) -> (F, F, F, F, F, F, F) {
    let t6728 = t4507 * t558;
    let t6730 = t6728 * t6723 * t593;
    let t6733 = t6723 * t352;
    let t6734 = t4515 * t6733;
    let t6737 = t4522 * t6733;
    let t6740 = t4479 * t1996;
    let t6743 = t4475 * t2010;
    let t6746 = -16.0 / 45.0 * t6690 - 8.0 / 15.0 * t1325 * t6693 + 8.0 / 135.0 * t6697 + 8.0 / 81.0 * t6700 + 8.0 / 135.0 * t6703 + 8.0 / 81.0 * t6706 + 16.0 / 135.0 * t6708 - t4948 + 16.0 / 45.0 * t4488 * t6713 + 16.0 / 45.0 * t4488 * t6717 - 8.0 / 27.0 * t4488 * t6720 - 16.0 / 45.0 * t3974 * t6725 + 16.0 / 45.0 * t4506 * t6730 + 16.0 / 45.0 * t4506 * t6734 - 8.0 / 27.0 * t4506 * t6737 - 16.0 / 45.0 * t3965 * t6740 - 16.0 / 45.0 * t3974 * t6743;
    (t6728, t6730, t6734, t6737, t6740, t6743, t6746)
}
