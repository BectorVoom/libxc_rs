//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 671/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk671<F: Float>(t1746: F, t6253: F, t1680: F, t520: F, t1743: F, t305: F, t1468: F, t534: F, t1782: F, t1792: F, t1837: F, t93: F) -> (F, F, F, F, F) {
    let t6254 = t1746 * t6253;
    let t6256 = t1680 * t1680;
    let t6258 = F::cast_from(1.0_f64) / t6256 / t520;
    let t6260 = t1743 * t1743;
    let t6261 = F::cast_from(1.0_f64) / t6260;
    let t6262 = t6261 * t305;
    let t6266 = t534 * t1468;
    let t6267 = t6266 * t1782;
    let t6268 = t1837 * t1792;
    let t6270 = t6267 * t93 * t6268;
    (t6254, t6258, t6262, t6267, t6270)
}
