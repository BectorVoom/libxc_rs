//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 753/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk753(t3201: f64, t8392: f64, t1588: f64, t920: f64, t1903: f64, t8217: f64, t1647: f64, t3199: f64, t1902: f64, t3170: f64, t487: f64, t492: f64) -> (f64, f64, f64, f64) {
    let t11826 = 2.0_f64 / 27.0_f64 * t8392 * t3201;
    let t11827 = t920 * t1588;
    let t11828 = t1903 * t11827;
    let t11829 = t8217 * t11828;
    let t11832 = t3199 * t1647;
    let t11833 = t1902 * t11832;
    let t11837 = t3170 * t487;
    let t11838 = t11837 * t492;
    (t11826, t11829, t11833, t11838)
}
