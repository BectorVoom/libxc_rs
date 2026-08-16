//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 719/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk719(t124: f64, t6856: f64, t123: f64, t616: f64, t1948: f64, t6560: f64, t121: f64, t2057: f64, t2061: f64, t2064: f64, t3411: f64, t641: f64, t642: f64, t6843: f64, t6847: f64, t6850: f64, t6855: f64) -> (f64, f64, f64, f64, f64) {
    let t6857 = t124 * t6856;
    let t6860 = t123 * t616;
    let t6861 = t6860 * t1948;
    let t6864 = t124 * t6560;
    let t6867 = -0.12897460341341234505e3_f64 * t6843 * t121 * t124 + 0.11607714307207111054e4_f64 * t6847 * t642 - 0.46430857228828444218e4_f64 * t6850 * t2061 + 0.11607714307207111054e4_f64 * t2057 * t2064 + 0.7738476204804740703e4_f64 * t6855 * t6857 - 0.46430857228828444218e4_f64 * t3411 * t6861 + 0.38692381024023703515e3_f64 * t641 * t6864;
    (t6857, t6860, t6861, t6864, t6867)
}
