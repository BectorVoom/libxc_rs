//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 716/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk716<F: Float>(t33: F, t265: F, t502: F, t2838: F, t3804: F, t1113: F, t1304: F, t2258: F, t3351: F, t504: F, t57: F, t606: F, t895: F, t3347: F, t1312: F, t2320: F, t2322: F, t2327: F, t2371: F, t670: F, t93: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t3805 = piecewise3(t503, t3804, t2838);
    let t3812 = piecewise3(t400, t2838 * t33 / 2.0 + t895 * t1113 + t265 * t3351 / 2.0, t3805 * t57 / 2.0 - t1304 * t606 - t504 * t2258 / 2.0);
    let t3813 = t3347 + t3812;
    let t3821 = 2.0 * t1312 * t2371 + 4.0 * t2322 * t670 + 2.0 * t2327 * t93 + t2320;
    (t3805, t3813, t3821)
}
