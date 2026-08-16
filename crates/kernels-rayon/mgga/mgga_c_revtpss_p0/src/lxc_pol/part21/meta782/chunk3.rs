//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2803/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2803(t10529: f64, t2782: f64, t51529: f64, t14602: f64, t2482: f64, t2811: f64, t4423: f64, t14575: f64, t2435: f64, t10943: f64, t14598: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64) {
    let t51531 = t2782 * t10529 * t51529;
    let t51535 = t2482 * t2811 * t4423 * t14602;
    let t51537 = t2435 * t14575;
    let t51538 = 0.21951497276451705329e-1_f64 * t51537;
    let t51541 = t14598 * t10943 * t72 * t686;
    (t51531, t51535, t51538, t51541)
}
