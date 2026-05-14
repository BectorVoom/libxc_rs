//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 888/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk888<F: Float>(t2185: F, t23658: F, t5900: F, t23657: F, t1647: F, t1969: F, t5899: F, t23602: F, t23606: F, t23613: F, t23616: F, t23619: F, t23623: F, t23627: F, t23629: F, t23634: F, t23639: F, t23643: F, t23647: F, t23650: F, t23655: F) -> (F, F, F, F, F) {
    let t23660 = t2185 * t5900 * t23658;
    let t23661 = t23657 * t23660;
    let t23663 = t1969 * t5900 * t1647;
    let t23664 = t5899 * t23663;
    let t23666 = -6.0 * t23602 - 3.0 * t23606 - 3.0 / 8.0 * t23613 - t23616 / 6.0 - 4.0 / 3.0 * t23619 + 2.0 * t23623 + t23627 - 2.0 / 3.0 * t23629 + t23634 / 2.0 + t23639 / 4.0 + t23643 / 6.0 + t23647 / 9.0 - t23650 / 9.0 + t23655 / 3.0 - t23661 - t23664 / 3.0;
    (t23660, t23661, t23663, t23664, t23666)
}
