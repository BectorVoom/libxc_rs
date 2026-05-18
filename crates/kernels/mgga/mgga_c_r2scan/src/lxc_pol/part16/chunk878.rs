//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 878/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk878<F: Float>(t44: F, t2892: F, t788: F, t5095: F, t785: F, t3190: F, t560: F, t551: F, t552: F, t1217: F, t3000: F, t3003: F, t415: F, t8571: F, t903: F, t99: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t9333 = t788 * t2892;
    let t9335 = t5095 * t785 * t9333;
    let t9337 = t3190 * t560;
    let t9339 = t551 * t552 * t9337;
    let t9353 = piecewise3::<f64>(t45, F::new(0.0), -F::new(10.0) / F::new(27.0) * t3000 * t415 + F::new(40.0) / F::new(9.0) * t903 * t1217 + F::new(10.0) / F::new(9.0) * t3003 * t415 + F::new(5.0) / F::new(3.0) * t99 * t8571);
    (t9335, t9339, t9353)
}
