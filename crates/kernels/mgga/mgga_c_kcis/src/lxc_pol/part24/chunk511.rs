//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 511/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk511<F: Float>(t1008: F, t4781: F, t1014: F, t1750: F, t1126: F, t1749: F, t303: F, t1800: F, t922: F, t3202: F, t3200: F, t1804: F, t3210: F, t1121: F, t1646: F, t3203: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4782 = t4781 * t1008;
    let t4787 = t1014 * t1750;
    let t4789 = t1749 * t1126;
    let t4790 = t303 * t4789;
    let t4792 = t1800 * t922;
    let t4793 = t3202 * t4792;
    let t4794 = t3200 * t4793;
    let t4796 = t1804 * t922;
    let t4797 = t3210 * t4796;
    let t4798 = t3200 * t4797;
    let t4800 = t1646 * t1121;
    let t4801 = t3203 * t4800;
    (t4782, t4787, t4789, t4790, t4792, t4793, t4794, t4796, t4797, t4798, t4801)
}
