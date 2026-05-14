//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1265/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1265<F: Float>(t11482: F, t11495: F, t11498: F, t11501: F, t125: F, t143: F, t15496: F, t1555: F, t1568: F, t1729: F, t1735: F, t18743: F, t18757: F, t18790: F, t18795: F, t18797: F, t18801: F, t18805: F, t18809: F, t18826: F, t18830: F, t18835: F, t2211: F, t2591: F, t2645: F, t2764: F, t2765: F, t2767: F, t2786: F, t2799: F, t2801: F, t2811: F, t5783: F, t6089: F, t770: F, t777: F, t8751: F) -> (F,) {
    let t18837 = (t15496 + t18743 + t18757 + t18790) * t125 - 0.10809180959278285 * t11482 + 0.039914113367515366 * t18795 - 2.0 * t777 * t18797 * t1555 + 6.0 * t18801 * t2811 - 3.0 * t2764 * t18805 - 6.0 * t18809 * t2767 - 3.0 * t5783 * t2765 * t770 * t1568 - 0.0005811348303577384 * t11495 - 0.0023245393214309535 * t11498 - 0.0017434044910732151 * t11501 + 3.0 * t6089 * t2801 - 1.849570964143173 * t8751 - t2645 * t2799 + 3.0 * t2211 * t2591 * t2786 + 12.0 * t1729 * t143 * t18826 + 6.0 * t18830 * t1735 - 0.10809180959278285 * t18835;
    (t18837,)
}
