//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 896/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk896(t2486: f64, t754: f64, t3893: f64, t3899: f64, t8392: f64, t2372: f64, t255: f64, t1131: f64, t761: f64, t2579: f64, t13832: f64, t13836: f64, t13840: f64, t13844: f64, t13849: f64, t13854: f64, t13860: f64, t13865: f64, t13869: f64, t13872: f64, t13875: f64, t13876: f64, t1901: f64, t193: f64, t3281: f64, t446: f64, t89: f64) -> f64 {
    let t13879 = t2486 * t754;
    let t13880 = t13879 * t3893;
    let t13884 = 2.0_f64 / 27.0_f64 * t8392 * t3899;
    let t13885 = t2372 * t255;
    let t13886 = t761 * t1131;
    let t13887 = t13886 * t2579;
    let t13888 = t13885 * t13887;
    let t13891 = -2.0_f64 / 3.0_f64 * t446 * t13832 - t446 * t13836 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t13840 + t89 * t193 * t13844 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t13849 - 2.0_f64 / 9.0_f64 * t1901 * t13854 - 2.0_f64 / 9.0_f64 * t1901 * t13860 - 2.0_f64 / 3.0_f64 * t1901 * t13865 + 2.0_f64 / 9.0_f64 * t3281 * t13869 - 4.0_f64 / 27.0_f64 * t13872 + t13875 + 4.0_f64 / 9.0_f64 * t1901 * t13876 - 4.0_f64 / 27.0_f64 * t1901 * t13880 - t13884 - 4.0_f64 / 3.0_f64 * t1901 * t13888;
    t13891
}
