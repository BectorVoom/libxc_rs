//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 924/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk924(t3265: f64, t3898: f64, t2381: f64, t3026: f64, t394: f64, t1249: f64, t3192: f64, t3214: f64, t3860: f64, t927: f64, t2886: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10050 = t3265 * t3898;
    let t10051 = t2381 * t10050;
    let t10054 = t394 * t3026;
    let t10055 = t1249 * t10054;
    let t10056 = t2381 * t10055;
    let t10059 = t3214 * t3192;
    let t10061 = t3860 * t927;
    let t10063 = t980 * t2886;
    (t10050, t10051, t10054, t10055, t10056, t10059, t10061, t10063)
}
