//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 627/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk627<F: Float>(t322: F, t1048: F, t3493: F, t499: F, t3275: F, t3352: F, t3465: F, t3356: F, t3367: F, t3359: F, t3361: F, t3364: F) -> (F, F, F, F, F, F) {
    let t324 = F::new(0.0) < t322;
    let t3495 = t1048 * t499 * t3493;
    let t3496 = t3495 / F::new(4.0);
    let t3498 = t3275 * t3465 * t3352;
    let t3499 = t3498 / F::new(4.0);
    let t3500 = F::new(2.0) / F::new(3.0) * t3356;
    let t3504 = F::new(2.0) / F::new(3.0) * t3367;
    let t3505 = t3500 + t3359 / F::new(4.0) - t3361 / F::new(4.0) + t3364 / F::new(2.0) + t3504;
    let t3506 = piecewise3::<f64>(t324, F::new(0.0), t3505);
    (t3496, t3499, t3500, t3504, t3505, t3506)
}
