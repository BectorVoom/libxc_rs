//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3694/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3694<F: Float>(t1260: F, t20850: F, t11262: F, t3600: F, t6630: F, t17225: F, t5391: F, t21183: F, t3172: F, t3711: F, t20875: F, t1042: F, t1261: F, t12809: F, t16771: F, t17199: F, t17204: F, t17344: F, t17412: F, t17550: F, t1789: F, t21028: F, t21257: F, t3584: F, t3714: F, t3720: F, t5270: F, t5296: F, t57136: F, t5825: F, t65370: F, t65947: F) -> F {
    let t69906 = t20850 * t1260;
    let t69910 = t3600 * t11262 * t6630;
    let t69916 = t5391 * t17225;
    let t69936 = t3711 * t3172 * t21183;
    let t69939 = t3711 * t3172 * t20875;
    let t69943 = -F::cast_from(0.76220476654346199062e-2_f64) * t1261 * t1042 * t57136 * t65947 + F::cast_from(0.28582678745379824648e-3_f64) * t69906 * t3714 - F::cast_from(0.95275595817932748827e-4_f64) * t69910 - F::cast_from(0.51448821741683684367e-2_f64) * t17344 * t1042 * t1789 * t16771 + F::cast_from(0.40650920882317972832e-2_f64) * t69916 + F::cast_from(0.30488190661738479624e-2_f64) * t5391 * t17199 + F::cast_from(0.91464571985215438872e-2_f64) * t5391 * t17204 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t1042 * t5296 * t5825 * t3584 + F::cast_from(0.85748036236139473944e-3_f64) * t12809 * t3720 * t21257 * t21028 + F::cast_from(0.14291339372689912324e-2_f64) * t1261 * t1042 * t17550 * t65370 + F::cast_from(0.19055119163586549765e-3_f64) * t69936 + F::cast_from(0.3811023832717309953e-3_f64) * t69939 + F::cast_from(0.60976381323476959248e-2_f64) * t17412 * t5270;
    t69943
}
