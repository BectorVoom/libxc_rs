//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 740/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk740(t12526: f64, t6915: f64, t6914: f64, t161: f64, t165: f64, t3116: f64, t2488: f64, t2487: f64, t912: f64, t587: f64, t12381: f64, t286: f64, t708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12527 = t6915 * t12526;
    let t12528 = t6914 * t12527;
    let t12531 = t161 * t165 * t3116;
    let t12532 = t2488 * t12531;
    let t12533 = t2487 * t12532;
    let t12535 = t912 * t12531;
    let t12536 = t587 * t12535;
    let t12538 = t912 * t12526;
    let t12539 = t587 * t12538;
    let t12541 = t2488 * t12526;
    let t12542 = t2487 * t12541;
    let t12555 = t12381 * t286 * t708;
    (t12527, t12528, t12531, t12532, t12533, t12535, t12536, t12538, t12539, t12541, t12542, t12555)
}
