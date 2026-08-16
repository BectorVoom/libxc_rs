//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1004/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1004(t75250: f64, t75254: f64, t75262: f64, t2211: f64, t41091: f64, t739: f64, t41006: f64, t884: f64, t1356: f64, t74292: f64, t8041: f64, t75271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77540 = 0.60611291211334054834e-6_f64 * t75250;
    let t77542 = 0.2727466165424534173e-1_f64 * t75254;
    let t77545 = 0.23268647941669485538e-4_f64 * t75262;
    let t77550 = 0.11974241701863808564e0_f64 * t739 * t2211 * t41091;
    let t77553 = 0.11974241701863808564e0_f64 * t884 * t2211 * t41006;
    let t77556 = 0.11974241701863808564e0_f64 * t1356 * t8041 * t74292;
    let t77557 = 0.20455996240684006296e-1_f64 * t75271;
    (t77540, t77542, t77545, t77550, t77553, t77556, t77557)
}
