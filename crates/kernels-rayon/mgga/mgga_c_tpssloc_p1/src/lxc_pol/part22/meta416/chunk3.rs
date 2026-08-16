//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1725/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1725(t1118: f64, t18834: f64, t1099: f64, t11185: f64, t6024: f64, t1128: f64, t6031: f64, t11211: f64, t11317: f64, t14702: f64, t15072: f64, t15074: f64, t18742: f64, t18747: f64, t18749: f64, t18752: f64, t18755: f64, t18757: f64) -> (f64, f64, f64, f64, f64) {
    let t18835 = t18834 * t1118;
    let t18837 = 1.0_f64 * t1099 * t18835;
    let t18839 = 0.16081979498692535067e2_f64 * t11185 * t6024;
    let t18840 = t6031 * t1128;
    let t18869 = 0.6311625e0_f64 * t18742 - t11317 + 0.45908888888888888888e0_f64 * t14702 - t15072 - t15074 + 0.11577222222222222222e0_f64 * t11211 - 0.157790625e0_f64 * t18747 + 0.6311625e0_f64 * t18749 + 0.31558125e0_f64 * t18752 + 0.264729375e1_f64 * t18755 - 0.3529725e1_f64 * t18757;
    (t18835, t18837, t18839, t18840, t18869)
}
