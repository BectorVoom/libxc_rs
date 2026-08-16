//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 622/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk622(t304: f64, t6478: f64, t1153: f64, t1757: f64, t1761: f64, t1780: f64, t3381: f64, t3392: f64, t348: f64, t365: f64, t368: f64, t5130: f64, t5151: f64, t6589: f64, t6593: f64, t6597: f64, t6601: f64, t6605: f64, t6641: f64, t6661: f64, t6665: f64, t6669: f64, t6673: f64, t86: f64) -> (f64, f64) {
    let t6676 = t304 * t6478;
    let t6680 = 0.619125e-2_f64 * t6641 * t348 + 0.1857375e-1_f64 * t1780 * t1757 - 0.123825e-1_f64 * t1780 * t1761 + 0.46434375e-2_f64 * t365 * t6589 - 0.1857375e-1_f64 * t3381 * t6593 + 0.9286875e-2_f64 * t365 * t6597 + 0.123825e-1_f64 * t365 * t6601 - 0.619125e-2_f64 * t365 * t6605 + t3392 - 0.35374814814814814814e-1_f64 * t5130 - 0.53062222222222222222e-1_f64 * t5151 - 0.44218518518518518518e-1_f64 * t1153 * t6661 - 0.53062222222222222222e-1_f64 * t1153 * t6665 + 0.53062222222222222222e-1_f64 * t1153 * t6669 - 0.26531111111111111111e-1_f64 * t1153 * t6673 - 0.39796666666666666666e-1_f64 * t86 * t368 * t6676;
    (t6676, t6680)
}
