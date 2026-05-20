//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3697/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3697<F: Float>(t1261: F, t20863: F, t3172: F, t20973: F, t3647: F, t21242: F, t3636: F, t12966: F, t17261: F, t17448: F, t17609: F, t17674: F, t17679: F, t17682: F, t17684: F, t21049: F, t21200: F, t21306: F, t3626: F, t44260: F, t5287: F, t5331: F, t5386: F, t5390: F, t56835: F, t56838: F, t6425: F, t6619: F) -> F {
    let t69984 = t1261 * t3172 * t20863;
    let t70006 = t3647 * t20973;
    let t70008 = t21242 * t3636;
    let t70011 = F::cast_from(0.63517063878621832552e-3_f64) * t69984 + F::cast_from(0.28582678745379824648e-3_f64) * t44260 * t6619 + F::cast_from(0.85748036236139473944e-3_f64) * t17609 * t5287 + F::cast_from(0.28582678745379824648e-3_f64) * t5331 * t3626 * t6425 * t17682 - F::cast_from(0.28582678745379824648e-3_f64) * t17448 * t17674 - F::cast_from(0.57165357490759649296e-3_f64) * t21049 * t17679 + F::cast_from(0.28582678745379824648e-3_f64) * t21306 * t17684 + F::cast_from(0.20325460441158986416e-2_f64) * t56835 + F::cast_from(0.17149607247227894789e-2_f64) * t17261 * t21200 - F::cast_from(0.91464571985215438872e-2_f64) * t12966 * t5390 * t5386 - F::cast_from(0.19055119163586549765e-3_f64) * t70006 + F::cast_from(0.20325460441158986416e-2_f64) * t70008 + F::cast_from(0.31758531939310916275e-3_f64) * t56838;
    t70011
}
