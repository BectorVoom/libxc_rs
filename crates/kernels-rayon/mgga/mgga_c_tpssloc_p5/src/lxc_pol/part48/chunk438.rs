//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 438/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk438(t1017: f64, t1030: f64, t1015: f64, t1012: f64, t1009: f64, t990: f64, t1011: f64, t1019: f64, t1004: f64, t1040: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2937: f64, t2939: f64, t2942: f64, t2946: f64, t2950: f64, t2954: f64) -> (f64, f64, f64, f64, f64) {
    let t3107 = t1030 * t1017;
    let t3108 = t1015 * t3107;
    let t3109 = t1012 * t3108;
    let t3112 = t990 * t1009;
    let t3113 = t3112 * t1011;
    let t3114 = t3113 * t1019;
    let t3117 = t1004 * t1040;
    let t3120 = -t2786 + t2789 - t2796 + t2839 + t2847 + t2937 + t2939 - t2942 + t2946 - t2950 - t2954;
    (t3109, t3112, t3114, t3117, t3120)
}
