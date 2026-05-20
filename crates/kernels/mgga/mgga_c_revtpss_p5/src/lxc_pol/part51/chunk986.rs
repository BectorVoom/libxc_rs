//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 986/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk986<F: Float>(t7742: F, t8634: F, t4147: F, t7933: F, t2034: F, t2014: F, t7937: F, t8568: F, t32098: F, t7900: F, t1519: F, t32162: F, t33575: F, t33578: F, t33580: F, t33583: F, t33584: F, t33587: F, t33589: F, t33592: F, t33595: F, t33599: F, t33600: F, t33603: F, t33605: F, t33647: F, t569: F, t651: F) -> (F, F, F, F) {
    let t33650 = F::new(4.0) * t8634 * t7742;
    let t33651 = t4147 * t7933;
    let t33652 = t2034 * t33651;
    let t33654 = F::new(2.0) * t2014 * t33652;
    let t33655 = t8568 * t7937;
    let t33657 = t32098 * t7900;
    let t33659 = F::new(3.0) * t2014 * t33657;
    let t33660 = -F::new(2.0) * t1519 * t32162 - F::new(2.0) * t33584 * t651 + t33647 * t569 - F::new(4.0) * t33575 - t33578 - t33580 - t33583 - F::new(4.0) * t33587 - F::new(4.0) * t33589 - F::new(4.0) * t33592 - t33595 - t33599 - F::new(4.0) * t33600 - F::new(4.0) * t33603 - F::new(4.0) * t33605 - t33650 - t33654 - F::new(2.0) * t33655 + t33659;
    (t33651, t33652, t33657, t33660)
}
