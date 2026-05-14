//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 662/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk662<F: Float>(t2220: F, t3888: F, t3893: F, t121: F, t766: F, t3141: F, t3857: F, t772: F, t2221: F, t3146: F, t3148: F, t2269: F, t3517: F, t119: F, t861: F, t120: F) -> (F, F, F, F, F, F) {
    let t7618 = t3888 * t2220;
    let t7619 = t7618 * t3893;
    let t7620 = t121 * t766;
    let t7621 = t3141 * t7620;
    let t7624 = t3857 * t2220;
    let t7625 = t7624 * t772;
    let t7628 = t2221 * t3146;
    let t7629 = t7628 * t3148;
    let t7633 = t2269 * t3517;
    let t7634 = t7633 * t119;
    let t7635 = t121 * t861;
    let t7636 = t120 * t7635;
    (t7619, t7621, t7625, t7629, t7634, t7636)
}
