//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1206/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206(t10047: f64, t225: f64, t2742: f64, t9587: f64, t9585: f64, t10046: f64, t10049: f64, t10104: f64, t10110: f64, t10112: f64, t10116: f64, t259: f64, t2591: f64, t2710: f64, t2713: f64, t2718: f64, t2719: f64, t2720: f64, t2743: f64, t798: f64, t855: f64, t866: f64, t9593: f64) -> f64 {
    let t40852 = t10047 * t225;
    let t40866 = t2742 * t2742;
    let t40870 = t9587 * t225;
    let t40875 = t9585 * t225;
    let t40887 = -36.0_f64 * t10110 * t2719 * t2742 * t855 + 4.0_f64 * t10046 * t259 * t798 + 6.0_f64 * t259 * t2591 * t2710 + 6.0_f64 * t2718 * t40866 * t855 + 12.0_f64 * t10049 * t2720 - 4.0_f64 * t10104 * t2713 - 24.0_f64 * t10112 * t2713 + 24.0_f64 * t10116 * t2713 + 24.0_f64 * t2720 * t9593 - 12.0_f64 * t2743 * t9593 - 4.0_f64 * t40852 * t866 - 12.0_f64 * t40870 * t866 - 4.0_f64 * t40875 * t866;
    t40887
}
