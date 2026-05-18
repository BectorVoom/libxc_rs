//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 885/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk885<F: Float>(t13277: F, t1433: F, t457: F, t12992: F, t1428: F, t13213: F, t13217: F, t13223: F, t13228: F, t13231: F, t13235: F, t13238: F, t13240: F, t13242: F, t1421: F, t456: F) -> (F, F, F) {
    let t13278 = t1433 * t13277;
    let t13279 = t457 * t13278;
    let t13282 = t1428 * t12992;
    let t13283 = t457 * t13282;
    let t13286 = -F::new(0.4435040025e-2) * t1421 * t13213 - F::new(0.4435040025e-2) * t1421 * t13217 + F::new(0.29201909629629629629e-2) * t1421 * t13223 + F::new(0.887008005e-2) * t1421 * t13228 - F::new(0.26281718666666666667e-2) * t13231 - F::new(0.65704296666666666666e-2) * t1421 * t13235 - F::new(0.43802864444444444445e-3) * t13238 + F::new(0.13140859333333333334e-2) * t13240 + F::new(0.21901432222222222222e-2) * t13242 - F::new(0.98556445e-3) * t456 * t13279 + F::new(0.1478346675e-2) * t456 * t13283;
    (t13278, t13282, t13286)
}
