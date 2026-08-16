//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 565/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk565(t10843: f64, t825: f64, t10627: f64, t701: f64, t7585: f64, t7584: f64, t326: f64, t2615: f64, t3474: f64, t5676: f64, t2610: f64, t2925: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10844 = t825 * t10843;
    let t10845 = 0.25561950635947166451e0_f64 * t10844;
    let t10847 = t10627 * t701;
    let t10848 = t7585 * t10847;
    let t10850 = 0.11502877786176224903e2_f64 * t7584 * t10848;
    let t10851 = t326 * t10847;
    let t10853 = 0.46011511144704899612e1_f64 * t2615 * t10851;
    let t10854 = t5676 * t3474;
    let t10855 = 0.14896037479937677779e-1_f64 * t10854;
    let t10856 = t2610 * t2925;
    (t10844, t10845, t10847, t10850, t10853, t10854, t10855, t10856)
}
