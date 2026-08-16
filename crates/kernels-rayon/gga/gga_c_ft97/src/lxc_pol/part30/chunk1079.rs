//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1079/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1079(t1882: f64, t35590: f64, t35726: f64, t10007: f64, t1131: f64, t11593: f64, t141727: f64, t14175: f64, t14182: f64, t142058: f64, t142074: f64, t142083: f64, t142117: f64, t149764: f64, t149870: f64, t151077: f64, t151362: f64, t1901: f64, t2360: f64, t242: f64, t2568: f64, t2574: f64, t28284: f64, t33274: f64, t33489: f64, t35678: f64, t3746: f64, t3821: f64, t3859: f64, t3875: f64, t3880: f64, t3886: f64, t446: f64, t6079: f64, t6154: f64, t6947: f64, t713: f64, t729: f64, t7502: f64, t7546: f64, t7553: f64, t762: f64) -> f64 {
    let t152082 = t1882 * t35590;
    let t152087 = t1882 * t35726;
    let t152095 = -2.0_f64 / 3.0_f64 * t446 * t242 * t149764 - t446 * t242 * t149870 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t142058 + t446 * t729 * t762 * t35678 * t713 / 3.0_f64 - t142074 + 4.0_f64 / 9.0_f64 * t11593 * t10007 * t7502 * t3746 - t1901 * t10007 * t141727 * t3875 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t14175 * t141727 * t3880 - 2.0_f64 / 9.0_f64 * t1901 * t14182 * t7553 * t2360 * t3886 + 4.0_f64 / 3.0_f64 * t446 * t2574 * t6947 * t6079 + 2.0_f64 / 3.0_f64 * t446 * t729 * t6154 * t28284 - 2.0_f64 / 3.0_f64 * t446 * t729 * t2568 * t7546 * t3821 - 2.0_f64 * t446 * t242 * t151077 + t142083 + t446 * t729 * t33274 * t3859 / 3.0_f64 + t152082 / 27.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t242 * t151362 - 2.0_f64 / 9.0_f64 * t152087 + t446 * t729 * t762 * t33489 * t1131 / 3.0_f64 - t142117 / 9.0_f64;
    t152095
}
