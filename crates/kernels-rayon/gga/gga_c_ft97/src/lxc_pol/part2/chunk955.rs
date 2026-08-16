//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 955/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk955(t13433: f64, t274: f64, t683: f64, t3750: f64, t688: f64, t231: f64, t1095: f64, t703: f64, t10328: f64, t2417: f64, t230: f64, t2380: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14818 = t683 * t13433 * t274;
    let t14825 = t3750 * t688;
    let t14826 = t14825 * t274;
    let t14827 = t231 * t14826;
    let t14832 = t703 * t1095;
    let t14833 = t14832 * t688;
    let t14834 = t14833 * t10328;
    let t14839 = t231 * t1095 * t2417 * t274;
    let t14842 = t230 * t1095;
    let t14844 = t2380 * t801 * t274;
    (t14818, t14827, t14834, t14839, t14842, t14844)
}
