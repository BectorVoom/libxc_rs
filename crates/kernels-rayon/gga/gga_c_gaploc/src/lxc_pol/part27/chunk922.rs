//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 922/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk922(t1967: f64, t9889: f64, t7810: f64, t7403: f64, t959: f64, t7340: f64, t3281: f64, t5676: f64, t2530: f64, t2610: f64, t2365: f64, t2033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    let t9935 = 0.29792074959875355558e-1_f64 * t7403 * t959;
    let t9937 = 0.29792074959875355558e-1_f64 * t7340 * t959;
    let t9942 = 0.29792074959875355558e-1_f64 * t5676 * t3281;
    let t9943 = t2610 * t2530;
    let t9944 = t2365 * t9943;
    let t9946 = 0.29792074959875355558e-1_f64 * t2033 * t9944;
    (t9890, t9891, t9935, t9937, t9942, t9943, t9944, t9946)
}
