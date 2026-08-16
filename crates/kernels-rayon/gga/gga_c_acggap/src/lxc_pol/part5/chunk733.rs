//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 733/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk733(t5385: f64, t871: f64, t5384: f64, t119: f64, t3932: f64, t3935: f64, t3939: f64, t5359: f64, t5361: f64, t5364: f64, t5365: f64, t5369: f64, t5372: f64, t5375: f64, t5381: f64, t5382: f64) -> (f64, f64, f64) {
    let t5386 = t5385 * t871;
    let t5388 = 0.26341796731742046394e1_f64 * t5384 * t5386;
    let t5390 = -t5359 - 0.13170898365871023197e1_f64 * t5361 + t5364 - 0.65854491829355115987e0_f64 * t5365 + 0.65854491829355115987e0_f64 * t5369 + 0.13170898365871023197e1_f64 * t5372 + 0.65854491829355115987e0_f64 * t119 * t5375 - t3932 + t5381 - 0.65854491829355115987e0_f64 * t5382 - t5388 - 0.13170898365871023197e1_f64 * t3935 + t3939;
    (t5386, t5388, t5390)
}
