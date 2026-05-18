//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 131/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk131<F: Float>(t51: F, t213: F, t413: F, t211: F, t254: F, t255: F, t260: F, t265: F, zeta_threshold: F) -> (F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t414 = t213 * t413;
    let t416 = t51 * t51;
    let t417 = piecewise3::<f64>(t52, t211, t416);
    let t418 = -F::new(0.32481568604919886) - t254 - t255 - t260 - t265;
    (t414, t416, t417, t418)
}
