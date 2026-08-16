//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 856/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk856(t27232: f64, t3366: f64, t13343: f64, t17288: f64, t13483: f64, t1377: f64, t10305: f64, t8045: f64, t13350: f64, t4349: f64, t605: f64, t1382: f64, t2497: f64, t3599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44676 = 4.0_f64 * t27232 * t3366;
    let t44678 = 6.0_f64 * t17288 * t13343;
    let t44679 = t1377 * t13483;
    let t44681 = 4.0_f64 * t8045 * t10305;
    let t44684 = 6.0_f64 * t4349 * t13350 * t605;
    let t44687 = 2.0_f64 * t1382 * t3599 * t2497;
    (t44676, t44678, t44679, t44681, t44684, t44687)
}
