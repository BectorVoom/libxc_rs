//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1176/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1176(t1389: f64, t6615: f64, t1349: f64, t138662: f64, t148210: f64, t148225: f64, t148880: f64, t148977: f64, t1969: f64, t26785: f64, t28: f64, t32714: f64, t32879: f64, t3450: f64, t34963: f64, t35033: f64, t35196: f64, t3588: f64, t379: f64, t5766: f64, t5772: f64, t5968: f64, t609: f64, t6708: f64, t7340: f64, t9432: f64, t9439: f64) -> f64 {
    let t149296 = t6615 * t1389;
    let t149301 = t5766 * t34963 / 6.0_f64 - 24.0_f64 * t9439 * t35033 * t609 - 12.0_f64 * t9439 * t35196 * t609 + t138662 - 24.0_f64 * t9439 * t6708 * t5968 + 4.0_f64 * t148210 + 4.0_f64 * t148880 + t1349 * t28 * t7340 * t3588 / 6.0_f64 + 8.0_f64 * t148977 + 4.0_f64 * t148225 + 2.0_f64 * t5772 * t9432 * t32879 * t3450 - t32714 * t26785 / 18.0_f64 - t5772 * t1969 * t149296 * t379 / 9.0_f64;
    t149301
}
