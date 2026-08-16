//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1170/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1170<F: Float>(t120199: F, t32014: F, t33764: F, t11921: F, t247: F, t33768: F, t8502: F, t120208: F, t120329: F, t120335: F, t120361: F, t120400: F, t120624: F, t120647: F, t126852: F, t1982: F, t25464: F, t27427: F, t27445: F, t3116: F, t31892: F, t31897: F, t31961: F, t31993: F, t33811: F, t4940: F, t4946: F, t988: F) -> F {
    let t126931 = t32014 * t120199 * t33764;
    let t126943 = t8502 * t247 * t11921 * t33768;
    let t126948 = -F::cast_from(0.11156198762715303246e-2_f64) * t120329 * t31993 * t3116 * t4940 - F::cast_from(0.11156198762715303246e-2_f64) * t120208 * t31993 * t3116 * t4946 - F::cast_from(0.52041769129231196772e1_f64) * t1982 * t120361 * t25464 * t4946 + F::cast_from(0.12548651892657985333e-3_f64) * t126931 - F::cast_from(0.17135921299530705785e1_f64) * t126852 * t31961 - F::cast_from(0.34694512752820797848e1_f64) * t120335 * t27445 - t120624 + F::cast_from(0.34271842599061411569e1_f64) * t31897 * t31892 * t33811 * t988 - F::cast_from(0.18822977838986977999e-3_f64) * t126943 + F::cast_from(0.34694512752820797848e1_f64) * t120400 * t27427 + F::cast_from(0.37645955677973955998e-3_f64) * t120647;
    t126948
}
