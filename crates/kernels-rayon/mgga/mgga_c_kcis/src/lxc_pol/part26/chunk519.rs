//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 519/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk519(t509: f64, t543: f64, t1419: f64, t1962: f64, t1319: f64, t1897: f64, t1317: f64, t1958: f64, t3820: f64, t5481: f64, t3795: f64, t3833: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5498 = t509 * t543;
    let t5499 = t1962 * t1419;
    let t5500 = t5498 * t5499;
    let t5503 = t1897 * t1319;
    let t5510 = t1317 * t1958;
    let t5513 = t3820 * t1897;
    let t5514 = t5513 * t1319;
    let t5516 = t1317 * t5481;
    let t5523 = -0.991e-2_f64 * t5514 + 0.1982e-1_f64 * t5516 + t3833 + 0.13758333333333333333e-2_f64 * t3795 + 0.13758333333333333333e-2_f64 * t5469 - 0.27516666666666666667e-2_f64 * t5472 + 0.8255e-2_f64 * t5475 + 0.8255e-2_f64 * t5479;
    (t5498, t5499, t5500, t5503, t5510, t5513, t5514, t5516, t5523)
}
