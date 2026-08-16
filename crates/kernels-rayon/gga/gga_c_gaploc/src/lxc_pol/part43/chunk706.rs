//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 706/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk706(t13157: f64, t1457: f64, t6060: f64, t1445: f64, t2087: f64, t10924: f64, t2558: f64, t9647: f64, t1029: f64, t3276: f64, t2508: f64, t3251: f64, t9014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13158 = t1457 * t13157;
    let t13160 = 0.21450293971110256001e1_f64 * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = 0.62115540045351614476e2_f64 * t2087 * t13161;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13184 = 0.64087718584518535698e-3_f64 * t13183;
    let t13185 = t3276 * t1029;
    let t13187 = 0.53833683610995569986e-1_f64 * t2508 * t13185;
    let t13191 = t9014 * t3251;
    (t13158, t13160, t13161, t13163, t13182, t13184, t13185, t13187, t13191)
}
