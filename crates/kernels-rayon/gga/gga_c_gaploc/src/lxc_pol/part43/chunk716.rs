//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 716/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk716(t2592: f64, t3749: f64, t123: f64, t3720: f64, t883: f64, t2685: f64, t2684: f64, t969: f64, t825: f64, t12653: f64, t12223: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13841 = t2592 * t3749;
    let t13846 = t3720 * t123;
    let t13847 = t13846 * t883;
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13855 = 0.38342925953920749677e0_f64 * t12653;
    let t13857 = t12223 * t935;
    (t13841, t13846, t13847, t13848, t13849, t13851, t13852, t13855, t13857)
}
