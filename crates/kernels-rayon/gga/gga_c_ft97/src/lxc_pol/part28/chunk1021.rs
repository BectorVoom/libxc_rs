//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1021/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1021(t3204: f64, t32350: f64, t22958: f64, t5674: f64, t137224: f64, t3188: f64, t22953: f64, t136269: f64, t93351: f64, t1871: f64, t22952: f64, t26006: f64, t5675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t144849 = t32350 * t3204;
    let t144851 = t5674 * t22958 * t144849;
    let t144853 = t137224 * t3188;
    let t144855 = t5674 * t22953 * t144853;
    let t144857 = t136269 * t3188;
    let t144859 = t5674 * t93351 * t144857;
    let t144863 = t22952 * t1871 * t5675 * t26006;
    (t144849, t144851, t144853, t144855, t144857, t144859, t144863)
}
