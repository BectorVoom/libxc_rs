//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3211/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3211<F: Float>(t13321: F, t13331: F, t1480: F, t21745: F, t21754: F, t2258: F, t2270: F, t2275: F, t2282: F, t2283: F, t2286: F, t44: F, t46090: F, t48: F, t56: F, t5835: F, t5838: F, t5843: F, t60: F, t60308: F, t60311: F, t60717: F, t60754: F, t60927: F, t614: F) -> F {
    let t60937 = -F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t614 * t21745 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t48 * t60754 - F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t5843 * t2286 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t5843 * t2283 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1480 * t13331 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t60 * t60754 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t56 * t2282 * t60717 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t2270 * t5835 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t44 * t2275 * t60717 + F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t2270 * t5838 - t46090 - F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t60308 * t60927 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t60311 * t60927 - F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t1480 * t13321 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t56 * t21754 * t2258;
    t60937
}
