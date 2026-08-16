//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3726/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3726<F: Float>(t1250: F, t12787: F, t16733: F, t17353: F, t17420: F, t17625: F, t17693: F, t17705: F, t17713: F, t17729: F, t17730: F, t17760: F, t20265: F, t20292: F, t21014: F, t44225: F, t57421: F, t57428: F, t59411: F, t70639: F, t70647: F, t70664: F, t70667: F, t70672: F) -> F {
    let t70675 = F::cast_from(0.25724410870841842183e-2_f64) * t70639 * t17713 - F::cast_from(0.47637797908966374414e-3_f64) * t17729 * t12787 * t20265 * t17730 + F::cast_from(0.5081365110289746604e-2_f64) * t70647 * t17760 - F::cast_from(0.17149607247227894789e-2_f64) * t17693 * t17353 * t1250 * t16733 - F::cast_from(0.67751534803863288054e-3_f64) * t57421 - F::cast_from(0.91464571985215438872e-2_f64) * t21014 * t17420 + F::cast_from(0.1270341277572436651e-2_f64) * t17729 * t44225 * t20292 * t17730 + F::cast_from(0.85748036236139473944e-3_f64) * t59411 * t17625 - F::cast_from(0.57165357490759649296e-3_f64) * t70664 + F::cast_from(0.28582678745379824648e-3_f64) * t70667 - F::cast_from(0.45732285992607719436e-2_f64) * t21014 * t17705 + F::cast_from(0.3811023832717309953e-3_f64) * t70672 + t57428 / F::cast_from(54.0_f64);
    t70675
}
