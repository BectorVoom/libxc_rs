//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1328/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1328<F: Float>(t25082: F, t30122: F, t33651: F, t18245: F, t7742: F, t114378: F, t1937: F, t30138: F, t7735: F, t29576: F, t7898: F, t30128: F, t4248: F) -> (F, F, F, F, F, F, F) {
    let t114415 = F::new(18.0) * t25082 * t33651 * t30122;
    let t114417 = F::new(6.0) * t18245 * t7742;
    let t114419 = F::new(6.0) * t114378 * t1937;
    let t114421 = F::new(12.0) * t30138 * t7735;
    let t114427 = F::new(6.0) * t7898 * t29576;
    let t114434 = F::new(12.0) * t30138 * t7742;
    let t114436 = F::new(6.0) * t4248 * t30128;
    (t114415, t114417, t114419, t114421, t114427, t114434, t114436)
}
