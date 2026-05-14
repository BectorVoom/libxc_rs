//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 663/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk663<F: Float>(t1062: F, t2270: F, t721: F, t119: F, t168: F, t2143: F, t609: F, t121: F, t4030: F, t741: F, t93: F, t89: F, t623: F, t2214: F, t2222: F, t2977: F, t3138: F, t3142: F, t3265: F, t709: F, t713: F, t7590: F, t7619: F, t7621: F, t7625: F, t7629: F, t7634: F, t7636: F) -> (F, F) {
    let t7639 = t2270 * t1062;
    let t7640 = t7639 * t721;
    let t7642 = t2270 * t119;
    let t7647 = t168 * t2143;
    let t7648 = t7647 * t609;
    let t7649 = t121 * t7648;
    let t7650 = t4030 * t7649;
    let t7652 = t93 * t741;
    let t7653 = t89 * t7652;
    let t7654 = t7647 * t623;
    let t7655 = t121 * t7654;
    let t7658 = 1.8805371096875316 * t3265 * t7590 + 2.9824072957409817 * t2222 * t3138 + 2.9824072957409817 * t2222 * t3142 - 0.15277772349540736 * t7619 * t7621 + 5.9648145914819635 * t7625 * t7621 + 2.9824072957409817 * t7629 - 4.937333717448355 * t2977 * t2214 - 4.937333717448355 * t7634 * t7636 - 4.937333717448355 * t7640 - 4.937333717448355 * t7642 * t713 - 4.937333717448355 * t7642 * t709 - 0.04115066352984959 * t7650 - 0.08230132705969918 * t7653 * t7655;
    (t7647, t7658)
}
