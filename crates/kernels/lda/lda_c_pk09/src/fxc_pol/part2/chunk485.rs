//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 485/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk485<F: Float>(t51: F, t2711: F, t424: F, t1191: F, t2146: F, t420: F, t1207: F, t1204: F, t1713: F, t425: F, t2140: F, t1197: F, t1193: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t2712 = t424 * t2711;
    let t2713 = t2712 * t1191;
    let t2715 = t420 * t2146;
    let t2716 = t2715 * t1207;
    let t2719 = t2711 * t1204 + F::new(1.28) * t1713 * t2716;
    let t2720 = t425 * t2719;
    let t2721 = piecewise3::<F>(t52, t2713, t2720);
    let t2723 = t420 * t2140;
    let t2724 = t2723 * t1197;
    let t2727 = t2711 * t1193 + F::new(1.28) * t1713 * t2724;
    (t2713, t2715, t2716, t2719, t2721, t2723, t2724, t2727)
}
