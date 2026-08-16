//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 548/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk548(t10544: f64, t6514: f64, t986: f64, t544: f64, t2386: f64, t2389: f64, t2898: f64, t10314: f64, t204: f64, t2476: f64, t594: f64, t1: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10545 = 0.25561950635947166451e0_f64 * t10544;
    let t10546 = t6514 * t986;
    let t10547 = t544 * t10546;
    let t10549 = 0.25025342966295298669e1_f64 * t10547 * t2386;
    let t10550 = t2898 * t2389;
    let t10551 = 0.29792074959875355558e-1_f64 * t10550;
    let t10552 = t204 * t10314;
    let t10554 = 0.46011511144704899612e1_f64 * t2476 * t10552;
    let t10555 = t594 * t986;
    let t10556 = t10555 * t1;
    (t10545, t10549, t10550, t10551, t10554, t10556)
}
