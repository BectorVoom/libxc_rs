//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1369/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1369<F: Float>(t30: F, t259: F, t379: F, t72363: F, t72411: F, t1289: F, t13335: F, t1819: F, t20577: F, t21702: F, t3431: F, t45: F, t4579: F, t581: F, t5870: F, t6374: F, t72203: F, t72242: F, t72277: F, t72317: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t72412 = t72363 + t72411;
    let t72413 = piecewise3::<F>(t380, F::cast_from(0.0_f64), t72412);
    let t72425 = piecewise3::<F>(t120, t72203 + t72242 + t72277 + t72317, t72413 * t45 / F::cast_from(2.0_f64) + t21702 * t581 / F::cast_from(2.0_f64) + t20577 * t1289 + t6374 * t3431 + t5870 * t4579 / F::cast_from(2.0_f64) + t1819 * t13335 / F::cast_from(2.0_f64));
    (t72412, t72425)
}
