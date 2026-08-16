//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1105/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1105(t3281: f64, t570: f64, t352: f64, t76244: f64, t77950: f64, t77955: f64, t77957: f64, t77963: f64, t77966: f64, t77969: f64, t77973: f64, t77976: f64, t77979: f64, t77982: f64, t77983: f64, t8940: f64) -> (f64, f64) {
    let t80444 = t3281 * t570;
    let t80449 = -t77950 + t77955 + t77957 - t77963 + t77966 + t77969 + t77973 + t77976 + 0.11974241701863808564e0_f64 * t8940 * t80444 * t352 + t77979 - t77982 + t77983 - 0.93188427318671584242e-2_f64 * t76244;
    (t80444, t80449)
}
