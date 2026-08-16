//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 644/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk644(t2365: f64, t8756: f64, t7390: f64, t3488: f64, t7354: f64, t2684: f64, t8769: f64, t6111: f64, t826: f64, t825: f64, t10813: f64, t10815: f64, t10819: f64, t10823: f64, t10825: f64, t10830: f64, t10831: f64, t2033: f64, t9789: f64, t9799: f64, t9803: f64, t9809: f64) -> f64 {
    let t10834 = t2365 * t8756;
    let t10835 = t7390 * t10834;
    let t10836 = 0.14896037479937677779e-1_f64 * t10835;
    let t10837 = t7354 * t3488;
    let t10838 = t2684 * t10837;
    let t10839 = 0.25561950635947166451e0_f64 * t10838;
    let t10840 = t2365 * t8769;
    let t10841 = t6111 * t10840;
    let t10842 = 0.29792074959875355558e-1_f64 * t10841;
    let t10843 = t826 * t3488;
    let t10844 = t825 * t10843;
    let t10845 = 0.25561950635947166451e0_f64 * t10844;
    let t10846 = t10813 - t10815 - t10819 - t10823 - t10825 - t10830 + 0.39722766613167140743e-1_f64 * t2033 * t10831 + t10836 - t10839 + t10842 + t10845 + t9789 - t9799 + t9803 - t9809;
    t10846
}
