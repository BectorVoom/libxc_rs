//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1436/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1436(t22479: f64, t3941: f64, t671: f64, t2363: f64, t6534: f64, t1873: f64, t55344: f64, t12524: f64, t23893: f64, t23896: f64, t12529: f64, t12532: f64, t2022: f64, t2319: f64, t23877: f64, t23880: f64, t577: f64, t7010: f64, t83973: f64, t83979: f64, t83980: f64, t83984: f64, t83988: f64, t83991: f64, t83993: f64, t83999: f64, t84001: f64, t84003: f64, t84004: f64, t9416: f64) -> f64 {
    let t84009 = 81.0_f64 * t3941 * t22479 * t671;
    let t84012 = 81.0_f64 * t3941 * t6534 * t2363;
    let t84014 = 81.0_f64 * t55344 * t1873;
    let t84016 = 162.0_f64 * t12524 * t23893;
    let t84018 = 81.0_f64 * t12524 * t23896;
    let t84019 = 0.45e1_f64 * t83973 * t577 + 81.0_f64 * t23880 * t12532 + t83979 + 81.0_f64 * t83980 * t2319 + t83984 + 27.0_f64 * t2022 * t12529 + t83988 + t83991 + t83993 + 0.405e2_f64 * t23877 * t2363 + 0.135e2_f64 * t7010 * t9416 + t83999 + t84001 + t84003 + 0.405e2_f64 * t84004 * t671 + t84009 + t84012 + t84014 + t84016 + t84018;
    t84019
}
