//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1269/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1269<F: Float>(t1449: F, t67701: F, t110845: F, t110859: F, t110872: F, t111478: F, t11593: F, t1175: F, t122658: F, t122662: F, t122667: F, t124331: F, t13830: F, t18690: F, t18698: F, t18702: F, t1901: F, t193: F, t241: F, t242: F, t24429: F, t2469: F, t258: F, t27742: F, t28404: F, t31197: F, t446: F, t5073: F, t6921: F, t729: F, t89: F) -> (F, F) {
    let t124337 = t67701 * t1449;
    let t124358 = 2.0 / 3.0 * t446 * t242 * t122658 + 4.0 / 3.0 * t446 * t242 * t122662 + 16.0 / 27.0 * t110845 + 2.0 / 3.0 * t446 * t242 * t122667 + 2.0 / 3.0 * t446 * t729 * t24429 * t5073 - 2.0 / 3.0 * t446 * t729 * t1175 * t27742 - t110859 + t89 * t193 * t241 * t124331 * t258 / 3.0 - t446 * t242 * t124337 / 3.0 + t446 * t729 * t2469 * t31197 / 3.0 - t110872 + 2.0 / 27.0 * t1901 * t28404 * t18690 - 10.0 / 81.0 * t1901 * t111478 * t18698 + 8.0 / 27.0 * t11593 * t28404 * t18702 + 2.0 / 3.0 * t446 * t729 * t13830 * t6921;
    (t124337, t124358)
}
