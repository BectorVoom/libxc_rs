//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1399/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1399(t5946: f64, t193: f64, t3216: f64, t336: f64, t4700: f64, t5950: f64, t60874: f64, t77157: f64, t77159: f64, t77224: f64, t77226: f64, t77229: f64, t77232: f64, t77236: f64, t77470: f64, t77474: f64, t77478: f64, t77482: f64) -> f64 {
    let t77924 = t5946 * t5946;
    let t77929 = -3.0_f64 * t193 * t3216 * t336 * t77924 + 12.0_f64 * t4700 * t5950 * t60874 + t77157 + t77159 - t77224 + t77226 - t77229 - t77232 + t77236 - t77470 + t77474 - t77478 - t77482;
    t77929
}
