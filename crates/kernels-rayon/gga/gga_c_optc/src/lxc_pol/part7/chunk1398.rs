//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1398/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1398(t1220: f64, t2367: f64, t8435: f64, t4281: f64, t9142: f64, t9240: f64, t11885: f64, t9243: f64, t8430: f64, t3274: f64, t9233: f64, t2839: f64, t2905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27843 = t1220 * t2367 * t8435;
    let t27846 = t4281 * t9142 * t9240;
    let t27849 = t4281 * t11885 * t9243;
    let t27856 = t1220 * t2367 * t8430;
    let t27858 = t3274 * t9233;
    let t27860 = t2905 * t2839;
    (t27843, t27846, t27849, t27856, t27858, t27860)
}
