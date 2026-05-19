//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1327/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1327<F: Float>(t33: F, t10326: F, t2159: F, t2258: F, t27048: F, t57: F, t606: F, t7677: F, t94325: F, t97508: F, t10192: F, t10260: F, t10263: F, t10416: F, t118: F, t2165: F, t2322: F, t2371: F, t27056: F, t27076: F, t27079: F, t569: F, t649: F, t651: F, t670: F, t7586: F, t7591: F, t7683: F, t92724: F, t92727: F, t92731: F, t92733: F, t92736: F, t94341: F, t94348: F, t94352: F, t94355: F, t96835: F, t96858: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t97518 = piecewise3::<F>(t400, t94325, t97508 * t57 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t27048 * t606 - F::new(3.0) / F::new(2.0) * t7677 * t2258 - t2159 * t10326 / F::new(2.0));
    let t97525 = -F::new(6.0) * t7586 * t10263 - F::new(12.0) * t2322 * t27076 + t96835 * t569 - t92724 - t92727 - t92731 - t92733 - t92736 - F::new(2.0) * t7586 * t10260 - F::new(6.0) * t651 * t27056 * t670 - F::new(6.0) * t651 * t7683 * t2371 - F::new(6.0) * t2322 * t27079 + t2165 * t10192 - t118 * (t96858 + t97518) - t94341 - F::new(3.0) * t649 * t27056 + t94348 - F::new(6.0) * t10416 * t7591 - t94352 - t94355;
    t97525
}
