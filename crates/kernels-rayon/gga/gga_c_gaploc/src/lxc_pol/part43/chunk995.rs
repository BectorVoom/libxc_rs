//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 995/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk995(t2386: f64, t3689: f64, t544: f64, t6514: f64, t4130: f64, t2482: f64, t9272: f64, t12063: f64, t1424: f64, t2299: f64, t10525: f64, t10526: f64, t47803: f64) -> (f64, f64, f64, f64) {
    let t47846 = t544 * t6514 * t3689 * t2386;
    let t47848 = t4130 * t3689;
    let t47850 = t9272 * t47848 * t2482;
    let t47854 = t544 * t2299 * t12063 * t1424;
    let t47860 = t10525 * t10526 * t47803;
    (t47846, t47850, t47854, t47860)
}
