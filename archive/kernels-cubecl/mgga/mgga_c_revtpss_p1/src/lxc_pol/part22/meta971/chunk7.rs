//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3251/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3251<F: Float>(t2722: F, t40325: F, t18481: F, t50768: F, t51176: F, t18333: F, t50769: F, t14547: F, t14894: F, t18426: F, t2430: F, t2477: F, t4364: F, t50415: F, t50757: F, t5962: F, t61660: F, t61669: F, t61673: F, t61675: F, t61677: F, t828: F, t851: F) -> (F, F) {
    let t61679 = t40325 * t2722;
    let t61689 = t50768 * t51176 * t18481;
    let t61692 = t50768 * t50769 * t18333;
    let t61694 = F::cast_from(0.10164000561857065645e-3_f64) * t50415 - F::cast_from(0.40015750243531754508e-1_f64) * t61660 + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t2477 * t828 * t5962 * t2430 + F::cast_from(0.28582678745379824648e-4_f64) * t61669 + F::cast_from(0.14291339372689912324e-4_f64) * t61673 + F::cast_from(0.54208002996571016773e-3_f64) * t61675 + F::cast_from(0.11337795902333997111e0_f64) * t61677 + F::cast_from(0.51448821741683684368e-2_f64) * t50757 * t4364 * t18426 * t61679 - F::cast_from(0.77173232612525526552e-2_f64) * t14894 * t4364 * t18426 * t14547 + F::cast_from(0.57165357490759649296e-3_f64) * t61689 - F::cast_from(0.11433071498151929859e-3_f64) * t61692;
    (t61679, t61694)
}
