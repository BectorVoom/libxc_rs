//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1336/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1336(t114812: f64, t1937: f64, t29508: f64, t7735: f64, t1907: f64, t6816: f64, t25082: f64, t8717: f64, t114768: f64, t114770: f64, t114773: f64, t114775: f64, t114779: f64, t114783: f64, t114785: f64, t114787: f64, t114790: f64, t114794: f64, t114803: f64, t114807: f64, t1502: f64, t1518: f64, t2007: f64, t22633: f64, t28030: f64, t29986: f64, t30119: f64, t4248: f64, t5921: f64, t651: f64) -> f64 {
    let t114814 = 2.0_f64 * t114812 * t1937;
    let t114816 = 6.0_f64 * t29508 * t7735;
    let t114820 = t6816 * t1907;
    let t114823 = 9.0_f64 * t25082 * t8717 * t114820;
    let t114824 = -6.0_f64 * t1518 * t29986 * t651 - 2.0_f64 * t2007 * t22633 * t651 - 3.0_f64 * t1502 * t29986 - 6.0_f64 * t28030 * t5921 - 6.0_f64 * t30119 * t4248 + t114768 + t114770 - t114773 + t114775 + t114779 + t114783 - t114785 - t114787 - t114790 + t114794 - t114803 + t114807 - t114814 - t114816 - t114823;
    t114824
}
