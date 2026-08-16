//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 990/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk990(t13669: f64, t30326: f64, t6043: f64, t7938: f64, t6059: f64, t1248: f64, t1249: f64, t30273: f64, t30294: f64, t4065: f64, t13607: f64, t30233: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30353 = t13669 * t30326;
    let t30355 = t6043 * t7938;
    let t30357 = t6059 * t7938;
    let t30360 = t1248 * t1249 * t30273;
    let t30363 = t1248 * t4065 * t30294;
    let t30366 = t1248 * t13607 * t30233;
    (t30353, t30355, t30357, t30360, t30363, t30366)
}
