//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1011/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1011(t10820: f64, t10915: f64, t10914: f64, t3473: f64, t549: f64, t2033: f64, t3040: f64, t9823: f64, t1022: f64, t2536: f64, t2021: f64, t2009: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10916 = t10915 * t10820;
    let t10918 = 0.21450293971110256001e1_f64 * t10914 * t10916;
    let t10919 = t549 * t3473;
    let t10920 = t2033 * t10919;
    let t10921 = 0.29792074959875355558e-1_f64 * t10920;
    let t10923 = 0.35750489951850426669e0_f64 * t9823 * t3040;
    let t10924 = t2536 * t1022;
    let t10925 = t2021 * t10924;
    let t10927 = 0.35750489951850426669e0_f64 * t10925 * t2009;
    (t10916, t10918, t10919, t10921, t10923, t10924, t10925, t10927)
}
