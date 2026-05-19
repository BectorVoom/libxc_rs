//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 578/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk578<F: Float>(t44: F, t51: F, t3190: F, t552: F, t551: F, t3016: F, t506: F, t529: F, t2999: F, t3002: F, t472: F, t99: F, t101: F, t3007: F, t3010: F, t476: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t3191 = t552 * t3190;
    let t3192 = t551 * t3191;
    let t3197 = t506 * t3016;
    let t3198 = t529 * t3197;
    let t3208 = piecewise3::<F>(t45, F::new(0.0), F::new(10.0) / F::new(9.0) * t472 * t2999 + F::new(5.0) / F::new(3.0) * t99 * t3002);
    let t3214 = piecewise3::<F>(t52, F::new(0.0), F::new(10.0) / F::new(9.0) * t476 * t3007 + F::new(5.0) / F::new(3.0) * t101 * t3010);
    (t3191, t3192, t3197, t3198, t3208, t3214)
}
