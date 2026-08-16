//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1131/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1131(t88734: f64, t88772: f64, t223: f64, t80002: f64, t52358: f64, t5005: f64, t1113: f64, t13414: f64, t13468: f64, t13469: f64, t13586: f64, t17847: f64, t17851: f64, t200: f64, t203: f64, t21319: f64, t21374: f64, t21393: f64, t237: f64, t2379: f64, t2384: f64, t2387: f64, t2710: f64, t3759: f64, t41593: f64, t41621: f64, t4957: f64, t4986: f64, t5026: f64, t66137: f64, t66328: f64, t66355: f64, t66419: f64, t66581: f64, t6758: f64, t678: f64, t680: f64, t79317: f64, t79423: f64, t80003: f64, t88493: f64, t88503: f64, t88504: f64, t88650: f64, t9524: f64, t9609: f64) -> (f64, f64, f64) {
    let t88773 = t88734 + t88772;
    let t88796 = t80002 * t223;
    let t88797 = t52358 * t88796;
    let t88805 = t5005 * t5005;
    let t88809 = -0.93019603785751168e-1_f64 * t3759 * t680 * t21374 * t1113 + 0.40531318161212073987e-5_f64 * t2710 * t88503 * t2384 - 0.23238868087529279928e-2_f64 * t13468 * t13469 * t21319 + 0.279058811357253504e0_f64 * t41593 * t79423 * t6758 - 0.81118562704294997116e-3_f64 * t17847 * t79317 + 0.1116235245429014016e-1_f64 * t2387 * t9609 * t88493 - 0.11627450473218896e-1_f64 * t678 * t680 * t88773 * t200 + 0.81118562704294997116e-3_f64 * t17851 * t66355 + 0.81118562704294997116e-3_f64 * t17851 * t66328 + 0.13510439387070691329e-4_f64 * t80003 * t13586 - 0.40559281352147498558e-3_f64 * t17851 * t66137 + 24.0_f64 * t4986 * t5026 + 0.20914981278776351936e-3_f64 * t678 * t41621 * t88504 + 0.279058811357253504e0_f64 * t66581 * t4957 + 0.46477736175058559857e-3_f64 * t2387 * t9524 * t88493 - 0.38465647900339007384e-5_f64 * t88797 * t13414 - 0.60903942508870095023e-4_f64 * t21393 * t66419 + 0.58097170218823199823e-3_f64 * t678 * t2379 * t88650 + 6.0_f64 * t203 * t88805 * t237;
    (t88773, t88796, t88809)
}
