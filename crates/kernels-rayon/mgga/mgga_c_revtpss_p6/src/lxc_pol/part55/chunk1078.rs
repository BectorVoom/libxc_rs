//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1078/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1078(t2055: f64, t7683: f64, t2163: f64, t7373: f64, t1310: f64, t2322: f64, t32112: f64, t32667: f64, t32671: f64, t32736: f64, t32740: f64, t33286: f64, t33287: f64, t33296: f64, t4254: f64, t508: f64, t569: f64, t651: f64, t671: f64, t7489: f64, t8764: f64, t8886: f64, t8892: f64) -> (f64, f64, f64) {
    let t33306 = t7683 * t2055;
    let t33311 = t2163 * t7373;
    let t33314 = -t1310 * t8886 - 2.0_f64 * t2322 * t8892 - t33286 * t508 - 2.0_f64 * t33287 * t671 + t33296 * t569 - 2.0_f64 * t33306 * t651 - 2.0_f64 * t33311 * t651 - 2.0_f64 * t4254 * t8892 + 3.0_f64 * t7489 * t8764 - t32112 + t32667 + t32671 + t32736 - t32740;
    (t33306, t33311, t33314)
}
