//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2048/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2048(t8107: f64, t9593: f64, t109077: f64, t109104: f64, t109150: f64, t109153: f64, t1453: f64, t18245: f64, t2056: f64, t2108: f64, t25082: f64, t26405: f64, t27153: f64, t27833: f64, t28167: f64, t28196: f64, t28198: f64, t28588: f64, t28709: f64, t29506: f64, t30122: f64, t30138: f64, t30584: f64, t30612: f64, t33183: f64, t34495: f64, t35927: f64, t5627: f64, t6934: f64, t7235: f64, t7367: f64, t7374: f64, t7484: f64, t7537: f64, t7539: f64, t7898: f64, t8109: f64, t86815: f64, t98450: f64) -> f64 {
    let t111176 = t8107 * t9593;
    let t111214 = t7484 * t6934 + 4.0_f64 * t28196 * t111176 * t28198 - 6.0_f64 * t98450 * t28588 + t30612 * t1453 - t29506 * t7539 - 6.0_f64 * t25082 * t34495 * t27153 + 2.0_f64 * t27833 * t8109 - 6.0_f64 * t25082 * t33183 * t30122 - 2.0_f64 * t7898 * t28709 - 2.0_f64 * t18245 * t7374 - 4.0_f64 * t109150 * t2056 - 4.0_f64 * t109153 * t2056 - 4.0_f64 * t30138 * t7367 - 12.0_f64 * t28167 * t26405 * t109104 - t7235 * t30584 - 6.0_f64 * t25082 * t26405 * t86815 + 12.0_f64 * t28167 * t35927 * t5627 + t109077 * t2108 + t29506 * t7537;
    t111214
}
