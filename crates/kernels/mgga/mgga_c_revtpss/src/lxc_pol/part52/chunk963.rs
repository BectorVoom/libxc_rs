//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 963/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk963<F: Float>(t32399: F, t32612: F, t32638: F, t32741: F, t3: F, t2042: F, t7547: F, t2113: F, t7331: F, t7334: F, t1459: F, t8731: F, t1936: F, t28974: F, t572: F, t26733: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32743 = 2.0 * t32399 + t32612 + t32638 + t32741;
    let t32744 = t3 * t32743;
    let t32755 = param_d * t32743;
    let t32760 = 3.0 * t7547 * t2042;
    let t32762 = 6.0 * t2113 * t7331;
    let t32764 = 3.0 * t2113 * t7334;
    let t32772 = 6.0 * t1459 * t8731;
    let t32773 = t28974 * t1936;
    let t32775 = 6.0 * t572 * t32773;
    let t32776 = t26733 * t1936;
    (t32743, t32744, t32755, t32760, t32762, t32764, t32772, t32773, t32775, t32776)
}
