//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 970/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk970(t10847: f64, t7585: f64, t7584: f64, t326: f64, t2615: f64, t3474: f64, t5676: f64, t2610: f64, t2925: f64, t2365: f64, t2033: f64, t6066: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10848 = t7585 * t10847;
    let t10850 = 0.11502877786176224903e2_f64 * t7584 * t10848;
    let t10851 = t326 * t10847;
    let t10853 = 0.46011511144704899612e1_f64 * t2615 * t10851;
    let t10854 = t5676 * t3474;
    let t10855 = 0.14896037479937677779e-1_f64 * t10854;
    let t10856 = t2610 * t2925;
    let t10857 = t2365 * t10856;
    let t10858 = t2033 * t10857;
    let t10859 = 0.14896037479937677779e-1_f64 * t10858;
    let t10860 = t6066 * t10847;
    (t10848, t10850, t10851, t10853, t10855, t10856, t10857, t10859, t10860)
}
