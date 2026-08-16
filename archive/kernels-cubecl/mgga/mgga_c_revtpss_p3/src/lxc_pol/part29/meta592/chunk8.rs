//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1975/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1975<F: Float>(t102409: F, t102411: F, t102422: F, t102434: F, t102439: F, t13920: F, t2097: F, t25930: F, t26304: F, t27868: F, t28855: F, t49376: F, t543: F, t7295: F, t7301: F, t7523: F, t96432: F, t96437: F, t97742: F, t97839: F, t97855: F, t98050: F, t98299: F) -> F {
    let t102443 = -F::cast_from(0.72280234901709995518e-2_f64) * t96432 - F::cast_from(0.96373646535613327357e-2_f64) * t102409 + F::cast_from(0.17135234354032049604e-1_f64) * t102411 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t26304 * t97742 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t26304 * t97839 - t102422 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t2097 * t13920 * t543 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t26304 * t98299 - F::cast_from(0.10975748638225852664e-1_f64) * t96437 + F::cast_from(0.17347256376410398924e1_f64) * t98050 * t7523 + F::cast_from(0.13009920719177044025e-1_f64) * t102434 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t26304 * t49376 - F::cast_from(0.11565819519348392139e-2_f64) * t102439 + F::cast_from(0.8673628188205199462e0_f64) * t97855 * t28855;
    t102443
}
