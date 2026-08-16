//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1024/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1024(t43522: f64, t33348: f64, t787: f64, t9824: f64, t10892: f64, t2021: f64, t7372: f64, t13042: f64, t2197: f64, t8793: f64, t9950: f64, t3040: f64, t41236: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43523 = 0.29792074959875355558e-1_f64 * t43522;
    let t43526 = t787 * t33348 * t9824;
    let t43527 = 0.29792074959875355558e-1_f64 * t43526;
    let t43529 = t2021 * t10892 * t7372;
    let t43567 = 0.43710935587469654631e2_f64 * t2197 * t13042;
    let t43569 = 0.10725146985555128001e1_f64 * t8793 * t9950;
    let t43571 = 0.35750489951850426669e0_f64 * t41236 * t3040;
    (t43523, t43527, t43529, t43567, t43569, t43571)
}
