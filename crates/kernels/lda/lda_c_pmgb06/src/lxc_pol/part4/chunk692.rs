//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 692/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk692<F: Float>(t1447: F, t1912: F, t1916: F, t1920: F, t1444: F, t1911: F, t2979: F, t493: F, t1586: F, t838: F, t1380: F, t2862: F, t224: F, t4622: F, t4624: F, t4626: F, t4628: F, t4630: F, t4684: F, t4713: F, t4717: F, t4718: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4721 = 4.0 / 135.0 * t1447 * t1912;
    let t4723 = 8.0 / 135.0 * t1447 * t1916;
    let t4725 = 4.0 / 81.0 * t1447 * t1920;
    let t4727 = 2.0 / 45.0 * t1444 * t1912;
    let t4728 = t2979 * t1911;
    let t4730 = 2.0 / 45.0 * t493 * t4728;
    let t4731 = t838 * t1586;
    let t4732 = t1380 * t4731;
    let t4734 = t493 * t4732 / 45.0;
    let t4735 = 4.0 / 135.0 * t2862;
    let t4736 = t4622 + t4624 + t4626 + t4628 + t4630 + t4684 - t4713 * t224 / 15.0 - t4717 + 2.0 / 135.0 * t4718 - t4721 - t4723 + t4725 - t4727 - t4730 - t4734 - t4735;
    (t4721, t4723, t4725, t4727, t4728, t4730, t4731, t4732, t4734, t4735, t4736)
}
