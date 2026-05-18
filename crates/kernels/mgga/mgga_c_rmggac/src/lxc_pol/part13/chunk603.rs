//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 603/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk603<F: Float>(t236: F, t830: F, t507: F, t2007: F, t2191: F, t1260: F, t1986: F, t675: F, t2004: F, t2186: F, t1263: F, t1223: F, t28: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7900 = t236 * t830;
    let t7901 = t507 * t7900;
    let t7903 = t2191 * t2007;
    let t7905 = t1986 * t1260;
    let t7906 = t675 * t7905;
    let t7908 = t2186 * t2004;
    let t7910 = t2186 * t2007;
    let t7912 = t2191 * t2004;
    let t7914 = t1986 * t1263;
    let t7915 = t675 * t7914;
    let t7919 = t1223 * t28;
    (t7900, t7901, t7903, t7905, t7906, t7908, t7910, t7912, t7914, t7915, t7919)
}
