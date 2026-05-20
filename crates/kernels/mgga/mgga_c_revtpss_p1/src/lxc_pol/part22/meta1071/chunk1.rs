//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3837/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3837<F: Float>(t13789: F, t13791: F, t13804: F, t13805: F, t13824: F, t1883: F, t22046: F, t23037: F, t3934: F, t48105: F, t48113: F, t48798: F, t5671: F, t5673: F, t73726: F, t73729: F, t73734: F, t73738: F, t73742: F, t73744: F, t73750: F) -> F {
    let t73752 = -F::cast_from(0.34299214494455789578e-2_f64) * t5671 * t13789 * t23037 * t13791 + F::cast_from(0.10289764348336736873e-1_f64) * t13804 * t13789 * t48105 * t48113 + F::cast_from(0.51448821741683684367e-1_f64) * t3934 * t48798 * t1883 * t13824 + F::cast_from(0.32012600194825403606e-1_f64) * t73726 + F::cast_from(0.22866142996303859718e-3_f64) * t73729 + F::cast_from(0.15246000842785598468e-3_f64) * t73734 + F::cast_from(0.10164000561857065645e-3_f64) * t73738 - F::cast_from(0.50820002809285328225e-4_f64) * t73742 + F::cast_from(0.12004725073059526352e-1_f64) * t73744 - F::cast_from(0.77173232612525526552e-2_f64) * t13804 * t5673 * t22046 * t13805 + F::cast_from(0.40015750243531754508e-1_f64) * t73750;
    t73752
}
