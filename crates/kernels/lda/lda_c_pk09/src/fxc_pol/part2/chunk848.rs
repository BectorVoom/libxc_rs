//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 848/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk848<F: Float>(t8705: F, t8718: F, t974: F, t89: F, t2152: F, t623: F, t844: F, t164: F, t1011: F, t2426: F, t4623: F, t4625: F, t4627: F, t709: F, t7792: F, t8679: F, t8682: F, t8684: F, t8686: F, t8689: F, t8691: F, t98: F) -> (F, F, F) {
    let t8719 = t8705 + t8718;
    let t8720 = t8719 * t974;
    let t8721 = t8720 * t89;
    let t8724 = t2152 * t623;
    let t8725 = t844 * t8724;
    let t8726 = t164 * t8725;
    let t8730 = F::new(12.992782516386768) * t8679 - t4623 + t4625 + t4627 + F::new(3.159189221415045) * t8682 - F::new(1.6183441301295518) * t8684 - F::new(1.6183441301295518) * t8686 + F::new(0.7897973053537612) * t8689 + F::new(1.6183441301295518) * t8691 + F::new(19.489173774580152) * t2426 * t1011 + F::new(19.489173774580152) * t8721 * t98 + F::new(22.07984838129906) * t8726 + F::new(2.427516195194328) * t7792 * t709;
    (t8720, t8726, t8730)
}
