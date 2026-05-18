//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 483/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk483<F: Float>(t3494: F, t439: F, t3356: F, t3413: F, t1178: F, t447: F, t1175: F, t300: F, t1203: F, t1208: F, t487: F, t1204: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3495 = F::new(1.0) / t3494;
    let t3496 = t439 * t3495;
    let t3503 = F::new(0.40256666666666666667e0) * t3356;
    let t3510 = F::new(0.137975e0) * t3413;
    let t3519 = t1178 * t1178;
    let t3520 = F::new(1.0) / t3519;
    let t3521 = t439 * t3520;
    let t3522 = t447 * t447;
    let t3523 = F::new(1.0) / t3522;
    let t3531 = t300 * t1175;
    let t3546 = F::new(0.11111111111111111111e-1) * t3356;
    let t3555 = t1203 * t1208;
    let t3556 = t3555 * t487;
    let t3561 = t1204 * t487;
    (t3495, t3496, t3503, t3510, t3520, t3521, t3523, t3531, t3546, t3555, t3556, t3561)
}
