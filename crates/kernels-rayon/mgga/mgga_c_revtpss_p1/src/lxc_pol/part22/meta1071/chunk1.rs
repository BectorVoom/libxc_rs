//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3837/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3837(t13789: f64, t13791: f64, t13804: f64, t13805: f64, t13824: f64, t1883: f64, t22046: f64, t23037: f64, t3934: f64, t48105: f64, t48113: f64, t48798: f64, t5671: f64, t5673: f64, t73726: f64, t73729: f64, t73734: f64, t73738: f64, t73742: f64, t73744: f64, t73750: f64) -> f64 {
    let t73752 = -0.34299214494455789578e-2_f64 * t5671 * t13789 * t23037 * t13791 + 0.10289764348336736873e-1_f64 * t13804 * t13789 * t48105 * t48113 + 0.51448821741683684367e-1_f64 * t3934 * t48798 * t1883 * t13824 + 0.32012600194825403606e-1_f64 * t73726 + 0.22866142996303859718e-3_f64 * t73729 + 0.15246000842785598468e-3_f64 * t73734 + 0.10164000561857065645e-3_f64 * t73738 - 0.50820002809285328225e-4_f64 * t73742 + 0.12004725073059526352e-1_f64 * t73744 - 0.77173232612525526552e-2_f64 * t13804 * t5673 * t22046 * t13805 + 0.40015750243531754508e-1_f64 * t73750;
    t73752
}
