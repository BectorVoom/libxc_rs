//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3700/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3700<F: Float>(t17608: F, t5292: F, t17547: F, t5265: F, t1261: F, t20906: F, t3172: F, t17416: F, t5391: F, t21272: F, t3636: F, t1042: F, t1252: F, t1260: F, t17550: F, t44264: F, t44270: F, t44273: F, t44276: F, t5268: F, t5384: F, t5386: F, t56246: F, t59241: F, t65829: F, t65947: F, t69875: F) -> F {
    let t70088 = t17608 * t5292;
    let t70091 = t17547 * t5265;
    let t70102 = t1261 * t3172 * t20906;
    let t70112 = t5391 * t17416;
    let t70114 = t21272 * t3636;
    let t70119 = -F::cast_from(0.45732285992607719436e-2_f64) * t70088 * t1252 - F::cast_from(0.30488190661738479624e-2_f64) * t70091 + F::cast_from(0.14291339372689912324e-2_f64) * t1261 * t1042 * t17550 * t65829 + F::cast_from(0.85748036236139473944e-2_f64) * t1261 * t1042 * t56246 * t65947 - F::cast_from(0.3811023832717309953e-3_f64) * t70102 - F::cast_from(0.57165357490759649296e-3_f64) * t5384 * t1042 * t5268 * t69875 + F::cast_from(0.1270341277572436651e-3_f64) * t44264 - F::cast_from(0.95275595817932748826e-4_f64) * t44270 - F::cast_from(0.47637797908966374413e-4_f64) * t44273 + F::cast_from(0.47637797908966374413e-4_f64) * t44276 - F::cast_from(0.33875767401931644027e-3_f64) * t70112 - F::cast_from(0.6436395806367012365e-2_f64) * t70114 + F::cast_from(0.17149607247227894789e-2_f64) * t59241 * t1260 * t5386;
    t70119
}
