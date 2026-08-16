//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 716/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk716(t1647: f64, t1893: f64, t5762: f64, t390: f64, t644: f64, t649: f64, t1664: f64, t1800: f64, t189: f64, t1658: f64, t5448: f64, t1957: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5763 = t1893 * t1647;
    let t5764 = t5762 * t5763;
    let t5766 = 0.2763462240212181411e2_f64 * t390 * t5764;
    let t5767 = t649 * t644;
    let t5768 = t5767 * t1664;
    let t5770 = 0.17183595094352973719e1_f64 * t390 * t5768;
    let t5771 = t189 * t1800;
    let t5772 = t1658 * t5771;
    let t5774 = 0.10685e0_f64 * t390 * t5772;
    let t5777 = 0.12822e1_f64 * t649 * t5448 * t189;
    let t5781 = t1957 * t206;
    (t5766, t5770, t5771, t5774, t5777, t5781)
}
