//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 988/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk988(t33590: f64, t6745: f64, t1454: f64, t668: f64, t33571: f64, t24237: f64, t35255: f64, t109652: f64, t6175: f64, t10052: f64, t1403: f64, t140583: f64, t140585: f64, t140588: f64, t140627: f64, t193: f64, t2347: f64, t2360: f64, t24231: f64, t28030: f64, t28036: f64, t35550: f64, t35639: f64, t3875: f64, t3886: f64, t5996: f64, t6002: f64, t6752: f64, t766: f64) -> (f64, f64) {
    let t149769 = t6745 * t33590;
    let t149771 = t1454 * t668;
    let t149790 = t6745 * t33571;
    let t149798 = t24237 * t35255;
    let t149800 = t109652 * t6175;
    let t149802 = t140583 / 54.0_f64 - t140585 / 18.0_f64 + t149769 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t6002 * t24231 * t149771 * t3875 + 2.0_f64 / 9.0_f64 * t6002 * t28030 * t1454 * t2360 * t3886 - 2.0_f64 / 27.0_f64 * t6002 * t28036 * t1454 * t2347 * t3886 + 2.0_f64 / 9.0_f64 * t140588 - 12.0_f64 * t10052 * t35639 * t766 - t149790 / 9.0_f64 - t1403 * t193 * t140627 * t6752 / 3.0_f64 - t5996 * t35550 / 3.0_f64 - t149798 / 27.0_f64 + 8.0_f64 * t149800;
    (t149800, t149802)
}
