//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 502/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk502(t2846: f64, t452: f64, t2094: f64, t2096: f64, t2098: f64, t2100: f64, t2733: f64, t2736: f64, t2803: f64, t2807: f64, t447: f64, t1748: f64, t2116: f64, t2121: f64, t2124: f64, t2783: f64, t2796: f64, t2813: f64, t2817: f64, t2826: f64, t2829: f64, t2832: f64, t2835: f64, t2838: f64, t455: f64, t463: f64) -> (f64, f64, f64, f64, f64) {
    let t2847 = t2846 * t452;
    let t2854 = t2094 - 1.4770435158815312_f64 * t2803 + t2096 + 1.4770435158815312_f64 * t2807 + t2098 - 0.2946275542389858_f64 * t2733 + t2100 + 0.2946275542389858_f64 * t2736;
    let t2855 = t447 * t2854;
    let t2856 = t2855 * t452;
    let t2859 = -t2796 * t1748 / 6.0_f64 + t463 * t2783 / 6.0_f64 - t2813 * t1748 / 6.0_f64 - 0.10237773105191754_f64 * t2736 + 0.14975624337724558_f64 * t2817 + 0.10237773105191754_f64 * t2733 + t2826 * t455 / 6.0_f64 + t2829 * t455 / 6.0_f64 + t2832 * t2116 / 12.0_f64 - t2835 * t455 / 6.0_f64 - t2838 * t455 / 6.0_f64 - t2847 * t455 / 6.0_f64 - t2856 * t455 / 6.0_f64 - t2121 - t2124;
    (t2847, t2854, t2855, t2856, t2859)
}
