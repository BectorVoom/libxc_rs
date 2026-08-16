//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1334/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1334(t2237: f64, t29034: f64, t1347: f64, t24774: f64, t3353: f64, t8865: f64, t3316: f64, t8854: f64, t20843: f64, t4114: f64, t4140: f64, t6569: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29036 = 0.16081979498692535067e2_f64 * t29034 * t2237;
    let t29038 = 2.0_f64 * t24774 * t1347;
    let t29040 = 4.0_f64 * t8865 * t3353;
    let t29042 = 2.0_f64 * t3316 * t8854;
    let t29044 = 2.0_f64 * t20843 * t4114;
    let t29046 = 1.0_f64 * t6569 * t4140;
    (t29036, t29038, t29040, t29042, t29044, t29046)
}
