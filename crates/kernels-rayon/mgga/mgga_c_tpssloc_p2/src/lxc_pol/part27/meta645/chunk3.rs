//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2210/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2210(t12606: f64, t3: f64, t1025: f64, t1933: f64, t1937: f64, t23453: f64, t23504: f64, t23515: f64, t23521: f64, t25588: f64, t25645: f64, t6722: f64, t6747: f64, t7573: f64, t7583: f64, t82927: f64, t82961: f64, t83111: f64, t88362: f64, t88367: f64, t88372: f64, t88385: f64, t88388: f64) -> f64 {
    let t88391 = t3 * t12606;
    let t88397 = t82961 / 2304.0_f64 - 0.20186378047070195428e-3_f64 * t88362 * t6747 - 0.20186378047070195428e-3_f64 * t88367 * t6747 - 0.10093189023535097714e-3_f64 * t25645 * t23504 - 0.20186378047070195428e-3_f64 * t88372 * t23515 + 0.10093189023535097714e-3_f64 * t88372 * t23521 + 0.72670960969452703541e-2_f64 * t23453 * t7573 * t1937 - 0.16149102437656156342e-2_f64 * t6722 * t25588 * t1937 - t88385 + 0.72670960969452703541e-2_f64 * t83111 * t7583 + t88388 * t1025 / 768.0_f64 + 0.10093189023535097714e-3_f64 * t1933 * t88391 * t1937 + 0.16149102437656156342e-2_f64 * t82927 * t7583;
    t88397
}
