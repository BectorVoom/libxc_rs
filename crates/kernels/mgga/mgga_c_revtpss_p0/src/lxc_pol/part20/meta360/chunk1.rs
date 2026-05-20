//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1310/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1310<F: Float>(t10665: F, t251: F, t2723: F, t2782: F, t4503: F, t10638: F, t10111: F, t22: F, t2789: F, t588: F, t870: F, t10963: F, t9303: F) -> (F, F, F, F, F, F) {
    let t39709 = t251 * t10665;
    let t39712 = t2782 * t4503 * t39709 * t2723;
    let t39714 = t251 * t10638;
    let t39719 = t10111 * t2789 * t22;
    let t39723 = F::cast_from(0.15709759505761725819e-2_f64) * t10111 * t870 * t588;
    let t39724 = t9303 * t10963;
    (t39709, t39712, t39714, t39719, t39723, t39724)
}
