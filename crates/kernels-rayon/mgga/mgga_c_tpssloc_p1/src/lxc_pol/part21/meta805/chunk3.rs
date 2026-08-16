//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2796/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2796(t232: f64, t58947: f64, t59072: f64, t13184: f64, t13193: f64, t13210: f64, t13251: f64, t13265: f64, t13302: f64, t13350: f64, t1510: f64, t16891: f64, t2643: f64, t2684: f64, t41116: f64, t4172: f64, t4180: f64, t4234: f64, t4250: f64, t4255: f64, t47039: f64, t47044: f64, t47047: f64, t47049: f64, t47079: f64, t47081: f64, t5619: f64, t58890: f64, t58900: f64, t58904: f64, t817: f64, t819: f64, t820: f64, t9613: f64) -> (f64, f64) {
    let t59074 = (t58947 + t59072) * t232;
    let t59088 = 7.0_f64 / 2304.0_f64 * t58890 + t47044 * t4250 / 192.0_f64 + t13251 * t13302 / 192.0_f64 - t2643 * t4180 * t16891 * t2684 / 3072.0_f64 - 7.0_f64 / 384.0_f64 * t58900 + t13251 * t13210 / 384.0_f64 - t58904 * t13265 / 256.0_f64 - 5.0_f64 / 192.0_f64 * t2643 * t13350 * t4234 * t4255 - 595.0_f64 / 5184.0_f64 * t47047 - 7.0_f64 / 12.0_f64 * t47049 + 5.0_f64 / 192.0_f64 * t4172 * t13193 - t817 * t819 * t820 * t59074 / 3072.0_f64 - t9613 * t5619 / 3072.0_f64 + 119.0_f64 / 1728.0_f64 * t41116 + 5.0_f64 / 64.0_f64 * t2643 * t47039 * t1510 * t13184 + 119.0_f64 / 864.0_f64 * t47079 - 7.0_f64 / 288.0_f64 * t47081;
    (t59074, t59088)
}
