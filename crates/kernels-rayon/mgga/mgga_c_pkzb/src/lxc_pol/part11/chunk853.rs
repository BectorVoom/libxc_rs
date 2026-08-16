//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 853/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk853(t5522: f64, t5543: f64, t7357: f64, t7451: f64, t9138: f64, t9140: f64, t9143: f64, t9148: f64, t9163: f64, t9165: f64, t9172: f64, t9174: f64) -> f64 {
    let t9176 = 0.19419375e1_f64 * t9138 - 0.258925e1_f64 * t9140 - 0.1294625e1_f64 * t9143 + 0.258925e1_f64 * t9165 - t5543 + 0.40256666666666666667e0_f64 * t5522 + 0.80513333333333333333e0_f64 * t7357 - t7451 - 0.301925e0_f64 * t9148 + 0.905775e0_f64 * t9163 - 0.412621875e-1_f64 * t9172 + 0.16504875e0_f64 * t9174;
    t9176
}
