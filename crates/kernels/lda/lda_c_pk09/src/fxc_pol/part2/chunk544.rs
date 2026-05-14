//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 544/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk544<F: Float>(t3190: F, t3767: F, t188: F, t733: F, t3743: F, t3745: F, t609: F, t891: F, t568: F, t948: F, t205: F, t148: F, t83: F, t142: F, t3163: F, t3498: F, t810: F) -> (F, F, F, F, F, F, F) {
    let t4584 = t3767 * t3190;
    let t4586 = t188 * t733;
    let t4587 = t4586 * t3743;
    let t4589 = t891 * t3745 * t609;
    let t4590 = t4587 * t4589;
    let t4594 = t568 * t948;
    let t4595 = t205 * t4594;
    let t4603 = t148 * t148;
    let t4604 = 1.0 / t4603;
    let t4609 = t83 * t733;
    let t4610 = t4609 * t142;
    let t4612 = 38.978347549160304 * t4610 * t3163;
    let t4614 = 25.985565032773536 * t810 * t3498;
    (t4584, t4587, t4590, t4595, t4604, t4612, t4614)
}
