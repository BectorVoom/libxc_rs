//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1385/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1385(t25257: f64, t3518: f64, t3579: f64, t1006: f64, t21676: f64, t2577: f64, t29996: f64, t29999: f64, t30002: f64, t30004: f64, t30007: f64, t30009: f64, t30012: f64, t30015: f64, t30018: f64, t30021: f64, t30024: f64, t30028: f64, t30031: f64, t30034: f64, t30038: f64, t4284: f64, t4297: f64, t7154: f64) -> (f64, f64, f64) {
    let t30040 = 0.64327917994770140268e2_f64 * t25257 * t3518;
    let t30041 = t3579 * t3579;
    let t30045 = -2.0_f64 * t21676 * t4284 + 1.0_f64 * t7154 * t4297 + t29996 + t29999 - t30002 + t30004 + t30007 - t30009 + t30012 + t30015 + t30018 + t30021 - t30024 - t30028 - t30031 - t30034 - t30038 - t30040 - 0.23392894490538584828e1_f64 * t2577 * t30041 * t1006;
    (t30040, t30041, t30045)
}
