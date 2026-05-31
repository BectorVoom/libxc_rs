//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1211/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1211<F: Float>(t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14433: F, t14618: F, t18571: F, t18572: F, t18573: F, t18574: F, t18578: F, t18579: F, t18581: F, t9524: F, t9542: F) -> (F, F) {
    let t18582 = F::cast_from(4.0_f64) * t10613;
    let t18583 = t14433 + t18571 - t9524 + t10592 + t18572 - t18573 - t10596 - t18574 + t18578 - t10604 + t9542 - t14618 + t18579 + t18581 - t10611 + t18582;
    (t18582, t18583)
}
