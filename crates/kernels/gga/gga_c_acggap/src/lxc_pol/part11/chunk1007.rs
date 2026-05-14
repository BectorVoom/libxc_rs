//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1007/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1007<F: Float>(t31362: F, t8956: F, t525: F, t839: F, t1165: F, t604: F, t7337: F, t7839: F, t8962: F, t31426: F, t31429: F, t35581: F, t35586: F, t35587: F, t35591: F, t35595: F, t35597: F, t35599: F, t35602: F, t35603: F, t35609: F, t35611: F, t35614: F) -> (F, F) {
    let t35616 = t31362 * t8956;
    let t35617 = 0.15724046144802076034e-2 * t35616;
    let t35618 = t525 * t839;
    let t35621 = t7337 * t1165 * t604 * t35618;
    let t35623 = t7839 * t8962;
    let t35624 = 0.62896184579208304136e-3 * t35623;
    let t35625 = t35581 - t35586 + 0.42874018118069736972e-3 * t35587 - 0.32155513588552302729e-2 * t35591 + t35595 + t35597 + 0.64311027177104605458e-2 * t35599 + t35602 + t35603 - 0.84046875e-1 * t31426 - 11.0 / 96.0 * t31429 + t35609 + t35611 - 0.47172138434406228102e-3 * t35614 - t35617 - 0.7862023072401038017e-3 * t35621 + t35624;
    (t35618, t35625)
}
