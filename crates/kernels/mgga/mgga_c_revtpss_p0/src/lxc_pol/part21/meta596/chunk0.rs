//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2315/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2315<F: Float>(t10578: F, t9575: F, t9572: F, t2434: F, t2496: F, t2629: F, t676: F, t9419: F, t9866: F, t123: F, t2390: F, t2630: F) -> (F, F, F, F, F, F, F, F) {
    let t39423 = t10578 * t9575;
    let t39425 = t10578 * t9572;
    let t39427 = t2434 * t2496;
    let t39429 = F::cast_from(0.12842595503380418954e1_f64) * t2629 * t39427;
    let t39430 = t676 * t9419;
    let t39432 = F::cast_from(0.38527786510141256862e1_f64) * t2629 * t39430;
    let t39433 = t10578 * t9866;
    let t39436 = t2390 * t123 * t2630;
    (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39436)
}
