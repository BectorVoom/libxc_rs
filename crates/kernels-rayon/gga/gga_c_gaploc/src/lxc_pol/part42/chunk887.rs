//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 887/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk887(t11936: f64, t3470: f64, t44045: f64, t2660: f64, t37180: f64, t2684: f64, t45369: f64, t7585: f64, t13589: f64, t15362: f64, t11823: f64, t22256: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45874 = 0.25025342966295298669e1_f64 * t11936 * t3470;
    let t45877 = 0.25561950635947166451e0_f64 * t44045;
    let t45882 = 0.10725146985555128001e1_f64 * t37180 * t2660;
    let t45885 = 0.14953741122029092374e3_f64 * t2684 * t7585 * t45369;
    let t45886 = t15362 * t13589;
    let t45887 = 0.29792074959875355558e-1_f64 * t45886;
    let t45888 = t11823 * t22256;
    (t45874, t45877, t45882, t45885, t45887, t45888)
}
