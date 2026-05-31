//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1751/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1751<F: Float>(t1196: F, t12552: F, t3523: F, t90357: F, t12248: F, t6470: F, t6474: F, t1732: F, t24324: F, t3384: F, t3433: F, t81650: F) -> (F, F, F, F) {
    let t90361 = F::cast_from(0.6233709278045326953e3_f64) * t1196 * t12552 * t90357 * t3523;
    let t90364 = F::cast_from(0.57895126195293126241e3_f64) * t12248 * t6474 * t6470;
    let t90367 = F::cast_from(8.0_f64) * t3384 * t24324 * t1732;
    let t90370 = F::cast_from(0.64327917994770140268e2_f64) * t3433 * t81650 * t1732;
    (t90361, t90364, t90367, t90370)
}
