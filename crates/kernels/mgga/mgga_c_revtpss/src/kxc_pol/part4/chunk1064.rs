//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1064/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1064<F: Float>(t57: F, t4186: F, t83: F, t13312: F, t1491: F, t2251: F, t2258: F, t4335: F, t606: F, t770: F, t14455: F, t1568: F, t785: F, t780: F, t2439: F, t212: F, t4469: F, zeta_threshold: F) -> (F, F, F) {
    let t155 = t57 <= zeta_threshold;
    let t14458 = t83 * t4186;
    let t14466 = piecewise3(t155, 0.0, -8.0 / 27.0 * t1491 * t2251 - 4.0 / 9.0 * t14458 * t606 - 2.0 / 9.0 * t4335 * t2258 - 2.0 / 3.0 * t770 * t13312);
    let t14468 = t14455 / 2.0 + t14466 / 2.0;
    let t14472 = t785 * t1568;
    let t14473 = t14472 * t780;
    let t14474 = t2439 * t14473;
    let t14476 = t212 * t4469;
    (t14468, t14474, t14476)
}
