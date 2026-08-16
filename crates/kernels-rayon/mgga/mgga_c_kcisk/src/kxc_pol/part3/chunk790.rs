//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 790/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk790(t10441: f64, t12198: f64, t5006: f64, t2023: f64, t5507: f64, t5515: f64, t7261: f64, t10449: f64, t2014: f64, t1775: f64, t3293: f64, t5491: f64) -> (f64, f64, f64, f64) {
    let t12199 = t12198 * t10441;
    let t12200 = t5006 * t12199;
    let t12203 = t5507 * t2023;
    let t12204 = t12203 * t5515;
    let t12205 = t7261 * t12204;
    let t12208 = t2014 * t10449;
    let t12209 = t1775 * t12208;
    let t12214 = t3293 * t2023;
    let t12215 = t5491 * t12214;
    (t12200, t12205, t12209, t12215)
}
