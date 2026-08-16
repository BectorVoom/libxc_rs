//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2565/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2565(t11478: f64, t4869: f64, t11282: f64, t1164: f64, t14854: f64, t4857: f64, t14961: f64, t3411: f64, t11311: f64, t1694: f64, t44154: f64, t11947: f64, t3637: f64, t4700: f64, t5091: f64, t51641: f64, t51669: f64, t51736: f64, t51738: f64, t51741: f64, t51744: f64) -> (f64, f64, f64, f64, f64) {
    let t51870 = 0.5848223622634646207e0_f64 * t4869 * t11478;
    let t51874 = 0.30762056574649219973e4_f64 * t1164 * t11282 * t4857 * t14854;
    let t51880 = 0.70178683471615754484e1_f64 * t3411 * t14961;
    let t51884 = 0.12304822629859687989e5_f64 * t1164 * t44154 * t1694 * t11311;
    let t51885 = 6.0_f64 * t11947 * t3637 * t4700 * t5091 + t51641 + t51669 + t51736 + t51738 + t51741 + t51744 - t51870 - t51874 + t51880 + t51884;
    (t51870, t51874, t51880, t51884, t51885)
}
