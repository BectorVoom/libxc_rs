//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1982/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1982<F: Float>(t102434: F, t102439: F, t102453: F, t102458: F, t102462: F, t102465: F, t108225: F, t108282: F, t108448: F, t22433: F, t25930: F, t26304: F, t27868: F, t28911: F, t28912: F, t75012: F, t7511: F, t75267: F, t7528: F, t96456: F, t96460: F) -> F {
    let t109704 = -F::cast_from(0.17347256376410398924e1_f64) * t108225 * t28912 + F::cast_from(0.26019841438354088051e-1_f64) * t102434 - F::cast_from(0.23131639038696784278e-2_f64) * t102439 + F::cast_from(0.45699670022203476294e-2_f64) * t96456 - t102453 - F::cast_from(0.39512695097613069591e1_f64) * t7511 * t22433 - t102458 + F::cast_from(0.14634331517634470219e-1_f64) * t102462 - t102465 + F::cast_from(0.13009920719177044025e-1_f64) * t96460 - F::cast_from(0.8673628188205199462e0_f64) * t27868 * t28911 * t75267 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t26304 * t108448 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t26304 * t75012 + F::cast_from(0.4336814094102599731e0_f64) * t108282 * t7528;
    t109704
}
