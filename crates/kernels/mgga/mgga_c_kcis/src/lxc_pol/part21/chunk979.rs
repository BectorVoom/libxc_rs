//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 979/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk979<F: Float>(t26840: F, t303: F, t3191: F, t356: F, t1014: F, t7727: F, t1087: F, t1134: F, t1086: F, t7731: F, t110: F, t2174: F, t2173: F, t3049: F, t3489: F, t7687: F, t7699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26841 = t303 * t26840;
    let t26843 = t356 * t3191;
    let t26844 = t303 * t26843;
    let t26846 = t1014 * t7727;
    let t26848 = t1087 * t1134;
    let t26849 = t303 * t26848;
    let t26851 = t1086 * t7731;
    let t26852 = t303 * t26851;
    let t26854 = t110 * t2174;
    let t26856 = 0.15445601851851851852e-3 * t2173 * t26854;
    let t26857 = t3049 * t3489;
    let t26860 = t7687 * t7699;
    (t26841, t26843, t26844, t26846, t26848, t26849, t26851, t26852, t26854, t26856, t26857, t26860)
}
