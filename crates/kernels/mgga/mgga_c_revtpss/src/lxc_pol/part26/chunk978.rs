//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 978/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk978<F: Float>(t33: F, t265: F, t502: F, t26625: F, t2085: F, t2258: F, t26665: F, t57: F, t606: F, t7468: F, t26633: F, t2051: F, t2327: F, t2107: F, t25177: F, t10416: F, t1312: F, t13435: F, t13440: F, t2055: F, t2322: F, t2371: F, t26153: F, t26210: F, t26399: F, t5523: F, t670: F, t7359: F, t7373: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t26666 = piecewise3(t503, 0.0, t26625);
    let t26673 = piecewise3(t400, t26665, t26666 * t57 / 2.0 - t7468 * t606 - t2085 * t2258 / 2.0);
    let t26674 = t26633 + t26673;
    let t26676 = t2051 * t2327;
    let t26679 = t2107 * t25177;
    let t26699 = 2.0 * t10416 * t2055 + 2.0 * t1312 * t26153 + 4.0 * t13435 * t2055 + 2.0 * t13440 * t2055 + 4.0 * t2322 * t7373 + 2.0 * t2371 * t7359 + 4.0 * t26399 * t670 + 4.0 * t5523 * t7373 + t26210 + 2.0 * t26676;
    (t26666, t26674, t26676, t26679, t26699)
}
