//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 483/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk483<F: Float>(t51: F, t2524: F, t2634: F, t2673: F, t2693: F, t213: F, t2146: F, t555: F, t1165: F, t1166: F, t1167: F, t1169: F, t1173: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t2695 = t2524 + t2634 + t2673 + t2693;
    let t2696 = t213 * t2695;
    let t2700 = piecewise3::<F>(t52, F::new(0.0), F::new(2.0) * t51 * t2146);
    let t2701 = t2700 * t555;
    let t2703 = t1165 + t1166 + t1167 + t1169 + t1173;
    (t2695, t2696, t2700, t2701, t2703)
}
