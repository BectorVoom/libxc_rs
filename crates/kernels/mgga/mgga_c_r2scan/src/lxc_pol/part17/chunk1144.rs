//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1144/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1144<F: Float>(t12198: F, t3270: F, t15059: F, t795: F, t40603: F, t1115: F, t2526: F, t12197: F, t1561: F, t12366: F, t12367: F, t12368: F) -> (F, F, F, F, F, F, F, F) {
    let t42234 = t3270 * t12198;
    let t42262 = t15059 * t795;
    let t42263 = t3270 * t42262;
    let t42274 = F::cast_from(0.3842256877732895568e-2_f64) * t40603;
    let t42318 = t3270 * t1115 * t2526;
    let t42331 = t1561 * t12197;
    let t42369 = F::new(2.0) * t12366;
    let t42370 = F::new(2.0) * t12367;
    let t42371 = F::new(2.0) * t12368;
    (t42234, t42263, t42274, t42318, t42331, t42369, t42370, t42371)
}
