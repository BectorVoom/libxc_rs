//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3697/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3697(t1261: f64, t20863: f64, t3172: f64, t20973: f64, t3647: f64, t21242: f64, t3636: f64, t12966: f64, t17261: f64, t17448: f64, t17609: f64, t17674: f64, t17679: f64, t17682: f64, t17684: f64, t21049: f64, t21200: f64, t21306: f64, t3626: f64, t44260: f64, t5287: f64, t5331: f64, t5386: f64, t5390: f64, t56835: f64, t56838: f64, t6425: f64, t6619: f64) -> f64 {
    let t69984 = t1261 * t3172 * t20863;
    let t70006 = t3647 * t20973;
    let t70008 = t21242 * t3636;
    let t70011 = 0.63517063878621832552e-3_f64 * t69984 + 0.28582678745379824648e-3_f64 * t44260 * t6619 + 0.85748036236139473944e-3_f64 * t17609 * t5287 + 0.28582678745379824648e-3_f64 * t5331 * t3626 * t6425 * t17682 - 0.28582678745379824648e-3_f64 * t17448 * t17674 - 0.57165357490759649296e-3_f64 * t21049 * t17679 + 0.28582678745379824648e-3_f64 * t21306 * t17684 + 0.20325460441158986416e-2_f64 * t56835 + 0.17149607247227894789e-2_f64 * t17261 * t21200 - 0.91464571985215438872e-2_f64 * t12966 * t5390 * t5386 - 0.19055119163586549765e-3_f64 * t70006 + 0.20325460441158986416e-2_f64 * t70008 + 0.31758531939310916275e-3_f64 * t56838;
    t70011
}
