//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1056/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1056<F: Float>(t33: F, t265: F, t502: F, t96072: F, t10326: F, t2085: F, t2258: F, t26666: F, t57: F, t606: F, t7468: F, t96121: F, t96166: F, t25876: F, t26304: F, t25894: F, t94398: F, t122: F, t72: F, t7506: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t96168 = piecewise3(t503, 0.0, t96072);
    let t96178 = piecewise3(t400, t96121 + t96166, t96168 * t57 / 2.0 - 3.0 / 2.0 * t26666 * t606 - 3.0 / 2.0 * t7468 * t2258 - t2085 * t10326 / 2.0);
    let t96186 = t25876 * t26304;
    let t96187 = t25894 * t96186;
    let t96188 = t96187 * t94398;
    let t96191 = t7506 * t72 * t122;
    (t96178, t96186, t96188, t96191)
}
