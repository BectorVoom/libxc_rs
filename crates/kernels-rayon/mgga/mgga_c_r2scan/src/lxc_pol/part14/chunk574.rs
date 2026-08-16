//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 574/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk574(t761: f64, t955: f64, t759: f64, t2060: f64, t897: f64, t2062: f64, t1421: f64, t1459: f64, t1463: f64, t1470: f64, t1480: f64, t1488: f64, t1511: f64, t1513: f64, t1526: f64, t1529: f64, t1533: f64, t246: f64, t2461: f64, t2490: f64, t2492: f64, t2494: f64, t2495: f64) -> (f64, f64) {
    let t2820 = t955 * t761;
    let t2821 = t759 * t2820;
    let t2823 = t2060 * t897;
    let t2824 = t2823 * t2062;
    let t2828 = t1421 - t1511 + 0.285764e-1_f64 * t2821 + t1459 - t1526 - 0.675260332e-1_f64 * t2824 - t1513 + t2490 + t2492 + t1470 - t1480 - t1488 - 0.285764e-1_f64 * t246 * t2461 - t2494 - t1529 + t1463 + t2495 - t1533;
    (t2823, t2828)
}
