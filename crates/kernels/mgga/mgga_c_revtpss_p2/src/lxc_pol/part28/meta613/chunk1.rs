//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2142/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2142<F: Float>(t13435: F, t7735: F, t2322: F, t27137: F, t1453: F, t1518: F, t25800: F, t28230: F, t651: F, t98567: F, t98569: F, t98571: F, t98574: F, t98578: F, t98581: F, t98584: F, t98590: F, t98594: F, t98597: F, t98599: F, t98601: F, t98603: F, t98605: F, t98607: F) -> F {
    let t98609 = F::new(4.0) * t13435 * t7735;
    let t98611 = F::new(4.0) * t2322 * t27137;
    let t98612 = -F::new(2.0) * t1518 * t25800 * t651 + F::new(2.0) * t1453 * t28230 + t98567 - t98569 - t98571 - t98574 + t98578 + t98581 - t98584 + t98590 + t98594 - t98597 - t98599 - t98601 - t98603 - t98605 - t98607 - t98609 - t98611;
    t98612
}
