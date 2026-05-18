//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 348/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk348<F: Float>(t322: F, t1292: F, t1295: F, t1300: F, t327: F, t833: F, t834: F, t330: F, t837: F, t1291: F) -> (F, F, F, F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t1305 = -F::new(0.64e0) * t1292 * t327 - F::new(0.128e1) * t1295 * t833 - F::new(0.128e1) * t1300 * t1295 - F::new(0.64e0) * t834 * t1292;
    let t1306 = t1305 * t330;
    let t1307 = t837 * t837;
    let t1308 = t1307 * t330;
    let t1310 = piecewise3::<f64>(t332, F::new(0.0), t1291);
    (t1305, t1306, t1307, t1308, t1310)
}
