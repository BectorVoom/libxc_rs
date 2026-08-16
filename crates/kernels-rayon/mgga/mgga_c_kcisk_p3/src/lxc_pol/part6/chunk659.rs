//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 659/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk659(t2647: f64, t5439: f64, t2014: f64, t7718: f64, t1775: f64, t5486: f64, t7715: f64, t5006: f64, t2642: f64, t5508: f64, t1586: f64, t20: f64, t8857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9162 = t2647 * t2647;
    let t9163 = t9162 * t5439;
    let t9168 = t2014 * t7718;
    let t9169 = t1775 * t9168;
    let t9172 = t5486 * t7715;
    let t9173 = t5006 * t9172;
    let t9176 = t2642 * t2642;
    let t9177 = t5508 * t9176;
    let t9178 = t1586 * t9177;
    let t9183 = t8857 * t20;
    (t9162, t9163, t9168, t9169, t9172, t9173, t9176, t9177, t9178, t9183)
}
