//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 914/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk914<F: Float>(t10673: F, t10687: F, t10692: F, t10870: F, t10900: F, t14712: F, t14716: F, t14761: F, t14765: F, t18338: F, t18340: F, t23253: F, t23257: F, t23263: F, t23267: F, t23275: F, t2721: F, t2730: F, t799: F) -> F {
    let t23278 = t10673 - F::cast_from(0.12862205435420921092e-2_f64) * t10870 * t23253 + F::cast_from(0.12862205435420921092e-2_f64) * t2721 * t23257 - F::cast_from(0.17006693853500995666e-1_f64) * t14712 + F::cast_from(0.40656002247428262579e-3_f64) * t14716 - t10900 * t23263 / F::cast_from(4.0_f64) - t799 * t23267 / F::cast_from(48.0_f64) - F::cast_from(0.13553694749236397037e-4_f64) * t14761 - t10687 + t10692 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t14765 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t18338 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t18340 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2730 * t23275;
    t23278
}
