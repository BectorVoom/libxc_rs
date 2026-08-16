//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 835/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk835(t28209: f64, t5184: f64, t5182: f64, t2364: f64, t9019: f64, t5192: f64, t15862: f64, t8485: f64, t9035: f64, t2441: f64, t7715: f64, t11682: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28210 = t5184 * t28209;
    let t28211 = t5182 * t28210;
    let t28217 = t9019 * t2364;
    let t28218 = t5192 * t28217;
    let t28219 = t5182 * t28218;
    let t28221 = t15862 * t8485;
    let t28222 = t5182 * t28221;
    let t28224 = t9035 * t2364;
    let t28225 = t5192 * t28224;
    let t28226 = t5182 * t28225;
    let t28228 = t7715 * t2441;
    let t28229 = t11682 * t28228;
    (t28211, t28219, t28222, t28226, t28228, t28229)
}
