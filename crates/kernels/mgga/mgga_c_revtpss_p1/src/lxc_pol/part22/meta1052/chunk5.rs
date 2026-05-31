//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3718/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3718<F: Float>(t1222: F, t140: F, t21209: F, t21213: F, t3685: F, t12865: F, t5436: F, t1012: F, t1225: F, t12866: F, t17634: F, t17661: F, t17693: F, t17696: F, t20771: F, t20937: F, t56981: F, t57191: F, t57209: F, t57212: F, t57214: F, t57222: F, t57227: F, t57378: F, t60754: F) -> (F, F) {
    let t70491 = t1222 * t140 * t21209;
    let t70493 = t21213 * t3685;
    let t70496 = t5436 * t12865;
    let t70508 = -t1222 * t1012 * t1225 * t60754 / F::cast_from(288.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t57191 + t57209 / F::cast_from(162.0_f64) + t57212 / F::cast_from(324.0_f64) - F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t57214 - t70491 / F::cast_from(432.0_f64) - F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t70493 + F::cast_from(0.19055119163586549765e-3_f64) * t57222 + F::cast_from(0.95275595817932748826e-3_f64) * t70496 * t17696 + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t17661 * t17634 - F::cast_from(0.76220476654346199061e-3_f64) * t57227 + F::cast_from(0.57165357490759649296e-3_f64) * t57378 * t20771 - F::cast_from(0.11433071498151929859e-2_f64) * t17693 * t56981 * t20937;
    (t70496, t70508)
}
