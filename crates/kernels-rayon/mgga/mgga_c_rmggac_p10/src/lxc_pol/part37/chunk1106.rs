//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1106/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1106(t3281: f64, t551: f64, t321: f64, t333: f64, t4669: f64, t5148: f64, t5266: f64, t76246: f64, t76250: f64, t76253: f64, t77988: f64, t77992: f64, t77995: f64, t77996: f64, t77997: f64, t77998: f64, t78005: f64, t78006: f64, t80444: f64) -> (f64, f64) {
    let t80452 = t3281 * t551;
    let t80462 = 0.15531404553111930707e-1_f64 * t76246 + 0.31062809106223861414e-2_f64 * t76250 + t76253 - 0.17961362552795712846e0_f64 * t4669 * t80452 * t333 - t77988 - t77992 - t77995 - 0.11974241701863808564e0_f64 * t5148 * t80444 * t321 + 0.11974241701863808564e0_f64 * t5266 * t80444 * t333 - t77996 - t77997 - t77998 + t78005 - t78006;
    (t80452, t80462)
}
