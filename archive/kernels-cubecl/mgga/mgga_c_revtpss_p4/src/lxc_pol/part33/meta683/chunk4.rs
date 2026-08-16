//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2244/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2244<F: Float>(t29019: F, t5273: F, t20973: F, t7624: F, t1785: F, t29082: F, t104727: F, t104739: F, t104742: F, t1252: F, t1266: F, t1808: F, t21200: F, t21267: F, t26852: F, t29037: F, t29040: F, t3670: F, t5386: F, t5397: F, t6631: F, t6673: F, t6683: F, t97206: F) -> F {
    let t112252 = t5273 * t29019;
    let t112258 = t7624 * t20973;
    let t112260 = t1785 * t29082;
    let t112278 = -F::cast_from(0.45732285992607719436e-2_f64) * t112252 * t1252 + F::cast_from(0.85748036236139473944e-3_f64) * t97206 * t6631 - F::cast_from(0.38110238327173099531e-3_f64) * t104742 - F::cast_from(0.19055119163586549765e-3_f64) * t112258 + F::cast_from(0.30488190661738479624e-2_f64) * t112260 * t1266 + F::cast_from(0.17149607247227894789e-2_f64) * t29040 * t21200 - F::cast_from(0.91464571985215438872e-2_f64) * t3670 * t29082 * t5386 + F::cast_from(0.47637797908966374413e-3_f64) * t26852 * t6673 - F::cast_from(0.57165357490759649296e-3_f64) * t26852 * t6683 + F::cast_from(0.30488190661738479624e-2_f64) * t104739 * t1808 - F::cast_from(0.57165357490759649296e-3_f64) * t29037 * t5397 - F::cast_from(0.25724410870841842183e-2_f64) * t104727 * t21267;
    t112278
}
