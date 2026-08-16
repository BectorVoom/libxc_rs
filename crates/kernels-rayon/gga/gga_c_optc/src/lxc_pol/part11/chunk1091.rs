//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1091/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1091(t2300: f64, t4831: f64, t2415: f64, t4895: f64, t5064: f64, t7274: f64, t999: f64, t2352: f64, t4851: f64, t1325: f64, t24442: f64, t24513: f64, t4941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39623 = t4831 * t2300;
    let t40005 = t4895 * t2415;
    let t40120 = t999 * t7274 * t5064;
    let t40188 = t4851 * t2352;
    let t40308 = t24442 * t1325;
    let t40326 = t24513 * t4941;
    (t39623, t40005, t40120, t40188, t40308, t40326)
}
