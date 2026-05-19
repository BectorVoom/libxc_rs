//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1024/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1024<F: Float>(t33: F, t265: F, t502: F, t11095: F, t12562: F, t13194: F, t10326: F, t1113: F, t1304: F, t2258: F, t2838: F, t3351: F, t3805: F, t504: F, t57: F, t606: F, t895: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t13196 = piecewise3::<F>(t503, t12562 + t13194, t11095);
    let t13206 = piecewise3::<F>(t400, t11095 * t33 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2838 * t1113 + F::new(3.0) / F::new(2.0) * t895 * t3351 + t265 * t9357 / F::new(2.0), t13196 * t57 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t3805 * t606 - F::new(3.0) / F::new(2.0) * t1304 * t2258 - t504 * t10326 / F::new(2.0));
    t13206
}
