//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 702/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk702(t11059: f64, t1866: f64, t446: f64, t1904: f64, t3008: f64, t7824: f64, t3104: f64, t375: f64, t89: f64, t358: f64, t463: f64, t2999: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11060 = t1866 * t11059;
    let t11061 = t446 * t11060;
    let t11064 = t3008 * t1904;
    let t11065 = t7824 * t11064;
    let t11066 = t446 * t11065;
    let t11069 = t89 * t375 * t3104;
    let t11070 = t11069 / 9.0_f64;
    let t11071 = t463 * t358;
    let t11073 = t89 * t2999 * t11071;
    (t11061, t11064, t11066, t11069, t11070, t11073)
}
