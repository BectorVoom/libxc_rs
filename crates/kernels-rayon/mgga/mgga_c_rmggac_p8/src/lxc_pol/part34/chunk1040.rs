//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1040/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1040(t5148: f64, t551: f64, t71910: f64, t14444: f64, t1587: f64, t76255: f64, t76258: f64, t76262: f64, t3203: f64, t570: f64, t1614: f64, t5266: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77992 = 0.11974241701863808564e0_f64 * t5148 * t71910 * t551;
    let t77995 = 0.11974241701863808564e0_f64 * t5148 * t14444 * t1587;
    let t77996 = 0.81823984962736025192e-1_f64 * t76255;
    let t77997 = 0.40911992481368012596e-1_f64 * t76258;
    let t77998 = 0.8182398496273602519e-1_f64 * t76262;
    let t77999 = t3203 * t570;
    let t78005 = 0.11974241701863808564e0_f64 * t5266 * t14444 * t1614;
    (t77992, t77995, t77996, t77997, t77998, t77999, t78005)
}
