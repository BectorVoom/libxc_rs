//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1048/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1048(t43925: f64, t10867: f64, t41511: f64, t25070: f64, t7427: f64, t9438: f64, t1022: f64, t9641: f64, t2009: f64, t2021: f64, t43683: f64, t7584: f64, t7585: f64) -> (f64, f64, f64, f64, f64) {
    let t43926 = 0.89376224879626066675e-1_f64 * t43925;
    let t43927 = t10867 * t41511;
    let t43928 = 0.89376224879626066675e-1_f64 * t43927;
    let t43930 = t7427 * t9438 * t25070;
    let t43931 = 0.47928657442400937096e-1_f64 * t43930;
    let t43932 = t9641 * t1022;
    let t43935 = 0.35750489951850426669e0_f64 * t2021 * t43932 * t2009;
    let t43938 = 0.11502877786176224903e2_f64 * t7584 * t7585 * t43683;
    (t43926, t43928, t43931, t43935, t43938)
}
