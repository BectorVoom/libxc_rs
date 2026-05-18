//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1078/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1078<F: Float>(t2055: F, t7683: F, t2163: F, t7373: F, t1310: F, t2322: F, t32112: F, t32667: F, t32671: F, t32736: F, t32740: F, t33286: F, t33287: F, t33296: F, t4254: F, t508: F, t569: F, t651: F, t671: F, t7489: F, t8764: F, t8886: F, t8892: F) -> (F, F, F) {
    let t33306 = t7683 * t2055;
    let t33311 = t2163 * t7373;
    let t33314 = -t1310 * t8886 - F::new(2.0) * t2322 * t8892 - t33286 * t508 - F::new(2.0) * t33287 * t671 + t33296 * t569 - F::new(2.0) * t33306 * t651 - F::new(2.0) * t33311 * t651 - F::new(2.0) * t4254 * t8892 + F::new(3.0) * t7489 * t8764 - t32112 + t32667 + t32671 + t32736 - t32740;
    (t33306, t33311, t33314)
}
