//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1142/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1142<F: Float>(t2408: F, t30: F, t605: F, t890: F, t2832: F, t1940: F, t1963: F, t2257: F, t2403: F, t25198: F, t25206: F, t25208: F, t25211: F, t25215: F, t25436: F, t25440: F, t25445: F, t4541: F, t7010: F, t7087: F, t7091: F, t7092: F) -> (F, F, F, F) {
    let t25446 = t30 * t2408;
    let t25449 = t605 * t890;
    let t25452 = t30 * t2832;
    let t25459 = F::cast_from(3.0_f64) * t4541 * t1963 * t25198 + F::cast_from(3.0_f64) * t2403 * t7087 * t7010 - F::cast_from(3.0_f64) * t25206 * t25208 + F::cast_from(3.0_f64) * t2403 * t1963 * t25211 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t25215 + t1940 * t25436 * t30 / F::cast_from(2.0_f64) - t1940 * t25440 * t7092 + t1940 * t7087 * t605 + t1940 * t25445 * t25446 - t1940 * t7091 * t25449 - t1940 * t7091 * t25452 / F::cast_from(2.0_f64) + t1940 * t1963 * t2257 / F::cast_from(2.0_f64);
    (t25446, t25449, t25452, t25459)
}
