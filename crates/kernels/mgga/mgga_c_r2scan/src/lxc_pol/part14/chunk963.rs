//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 963/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk963<F: Float>(t322: F, t11216: F, t3506: F, t833: F, t1120: F, t1299: F, t1295: F, t829: F, t1292: F, t1300: F, t327: F, t3509: F, t6693: F, t834: F) -> (F, F, F, F) {
    let t324 = F::new(0.0) < t322;
    let t11217 = piecewise3::<F>(t324, F::new(0.0), t11216);
    let t11220 = t3506 * t833;
    let t11223 = t1120 * t1299;
    let t11228 = t1120 * t1295;
    let t11231 = t3506 * t829;
    let t11234 = t1120 * t1292;
    let t11239 = -F::new(0.64e0) * t11217 * t327 - F::new(0.256e1) * t11220 * t829 - F::new(0.384e1) * t11223 * t1295 - F::new(0.128e1) * t3509 * t1292 - F::new(0.384e1) * t6693 * t11228 - F::new(0.256e1) * t1300 * t11231 - F::new(0.128e1) * t1300 * t11234 - F::new(0.64e0) * t834 * t11217;
    (t11217, t11220, t11223, t11239)
}
