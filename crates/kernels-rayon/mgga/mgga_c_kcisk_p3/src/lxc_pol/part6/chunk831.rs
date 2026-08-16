//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 831/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk831(t4265: f64, t8216: f64, t442: f64, t8159: f64, t140: f64, t299: f64, t8227: f64, t240: f64, t7796: f64, t1528: f64, t8344: f64, t4463: f64, t8365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27321 = t4265 * t8216;
    let t27331 = t8159 * t442;
    let t27355 = t140 * t299 * t8227;
    let t27491 = t240 * t7796;
    let t27516 = t8344 * t1528;
    let t27584 = t8365 * t4463;
    (t27321, t27331, t27355, t27491, t27516, t27584)
}
