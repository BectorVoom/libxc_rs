//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1170/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1170(t120199: f64, t32014: f64, t33764: f64, t11921: f64, t247: f64, t33768: f64, t8502: f64, t120208: f64, t120329: f64, t120335: f64, t120361: f64, t120400: f64, t120624: f64, t120647: f64, t126852: f64, t1982: f64, t25464: f64, t27427: f64, t27445: f64, t3116: f64, t31892: f64, t31897: f64, t31961: f64, t31993: f64, t33811: f64, t4940: f64, t4946: f64, t988: f64) -> f64 {
    let t126931 = t32014 * t120199 * t33764;
    let t126943 = t8502 * t247 * t11921 * t33768;
    let t126948 = -0.11156198762715303246e-2_f64 * t120329 * t31993 * t3116 * t4940 - 0.11156198762715303246e-2_f64 * t120208 * t31993 * t3116 * t4946 - 0.52041769129231196772e1_f64 * t1982 * t120361 * t25464 * t4946 + 0.12548651892657985333e-3_f64 * t126931 - 0.17135921299530705785e1_f64 * t126852 * t31961 - 0.34694512752820797848e1_f64 * t120335 * t27445 - t120624 + 0.34271842599061411569e1_f64 * t31897 * t31892 * t33811 * t988 - 0.18822977838986977999e-3_f64 * t126943 + 0.34694512752820797848e1_f64 * t120400 * t27427 + 0.37645955677973955998e-3_f64 * t120647;
    t126948
}
