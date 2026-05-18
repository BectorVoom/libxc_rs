//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1131/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1131<F: Float>(t88734: F, t88772: F, t223: F, t80002: F, t52358: F, t5005: F, t1113: F, t13414: F, t13468: F, t13469: F, t13586: F, t17847: F, t17851: F, t200: F, t203: F, t21319: F, t21374: F, t21393: F, t237: F, t2379: F, t2384: F, t2387: F, t2710: F, t3759: F, t41593: F, t41621: F, t4957: F, t4986: F, t5026: F, t66137: F, t66328: F, t66355: F, t66419: F, t66581: F, t6758: F, t678: F, t680: F, t79317: F, t79423: F, t80003: F, t88493: F, t88503: F, t88504: F, t88650: F, t9524: F, t9609: F) -> (F, F, F) {
    let t88773 = t88734 + t88772;
    let t88796 = t80002 * t223;
    let t88797 = t52358 * t88796;
    let t88805 = t5005 * t5005;
    let t88809 = -F::new(0.93019603785751168e-1) * t3759 * t680 * t21374 * t1113 + F::new(0.40531318161212073987e-5) * t2710 * t88503 * t2384 - F::new(0.23238868087529279928e-2) * t13468 * t13469 * t21319 + F::new(0.279058811357253504e0) * t41593 * t79423 * t6758 - F::new(0.81118562704294997116e-3) * t17847 * t79317 + F::new(0.1116235245429014016e-1) * t2387 * t9609 * t88493 - F::new(0.11627450473218896e-1) * t678 * t680 * t88773 * t200 + F::new(0.81118562704294997116e-3) * t17851 * t66355 + F::new(0.81118562704294997116e-3) * t17851 * t66328 + F::new(0.13510439387070691329e-4) * t80003 * t13586 - F::new(0.40559281352147498558e-3) * t17851 * t66137 + F::new(24.0) * t4986 * t5026 + F::new(0.20914981278776351936e-3) * t678 * t41621 * t88504 + F::new(0.279058811357253504e0) * t66581 * t4957 + F::new(0.46477736175058559857e-3) * t2387 * t9524 * t88493 - F::new(0.38465647900339007384e-5) * t88797 * t13414 - F::new(0.60903942508870095023e-4) * t21393 * t66419 + F::new(0.58097170218823199823e-3) * t678 * t2379 * t88650 + F::new(6.0) * t203 * t88805 * t237;
    (t88773, t88796, t88809)
}
