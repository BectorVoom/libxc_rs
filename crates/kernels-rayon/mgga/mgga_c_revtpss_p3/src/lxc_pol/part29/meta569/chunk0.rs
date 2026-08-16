//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1916/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1916(t1468: f64, t2832: f64, t2408: f64, t25207: f64, t61182: f64, t2430: f64, t1583: f64, t2257: f64, t2394: f64, t11064: f64, t605: f64, t27384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98736 = t1468 * t2832;
    let t98740 = t1468 * t2408;
    let t98743 = t25207 * t61182;
    let t98751 = t1468 * t2430;
    let t98755 = t2257 * t1583;
    let t98759 = t1583 * t2394;
    let t98760 = t25207 * t98759;
    let t98763 = t11064 * t605;
    let t98764 = t98763 * t27384;
    (t98736, t98740, t98743, t98751, t98755, t98759, t98760, t98764)
}
