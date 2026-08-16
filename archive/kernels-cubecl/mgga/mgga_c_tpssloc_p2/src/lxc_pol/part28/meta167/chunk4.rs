//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 825/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk825<F: Float>(t25: F, t1268: F, t2312: F, t2314: F, t2319: F, t2363: F, t671: F, t88: F, t526: F, t606: F, t2249: F, t514: F, t528: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t3660 = F::cast_from(2.0_f64) * t1268 * t2363 + F::cast_from(4.0_f64) * t2314 * t671 + F::cast_from(2.0_f64) * t2319 * t88 + t2312;
    let t3664 = F::cast_from(1.0_f64) / t526;
    let t3665 = t606 * t606;
    let t3671 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3664 * t3665 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t514 * t2249);
    let t3672 = F::cast_from(1.0_f64) / t528;
    (t3660, t3664, t3665, t3671, t3672)
}
