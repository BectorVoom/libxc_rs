//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 912/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk912(t326: f64, t43508: f64, t825: f64, t2684: f64, t7585: f64, t33360: f64, t787: f64, t9824: f64, t33348: f64, t13042: f64, t2197: f64, t8793: f64, t9950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43511 = 0.92023022289409799224e1_f64 * t825 * t326 * t43508;
    let t43514 = 0.43710935587469654631e2_f64 * t2684 * t7585 * t43508;
    let t43522 = t787 * t33360 * t9824;
    let t43523 = 0.29792074959875355558e-1_f64 * t43522;
    let t43526 = t787 * t33348 * t9824;
    let t43527 = 0.29792074959875355558e-1_f64 * t43526;
    let t43567 = 0.43710935587469654631e2_f64 * t2197 * t13042;
    let t43569 = 0.10725146985555128001e1_f64 * t8793 * t9950;
    (t43511, t43514, t43523, t43527, t43567, t43569)
}
