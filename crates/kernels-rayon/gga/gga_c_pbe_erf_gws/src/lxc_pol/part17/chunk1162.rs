//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1162/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1162(t3298: f64, t3975: f64, t3972: f64, t4161: f64, t840: f64, t2409: f64, t9707: f64, t3965: f64, t11434: f64, t13776: f64, t8582: f64, t1192: f64, t6126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14776 = t3975 * t3298;
    let t14777 = t3972 * t14776;
    let t14779 = t840 * t4161;
    let t14781 = t2409 * t9707;
    let t14782 = t3965 * t14781;
    let t14784 = t3975 * t11434;
    let t14785 = t13776 * t14784;
    let t14787 = t2409 * t8582;
    let t14788 = t3965 * t14787;
    let t14791 = t6126 * t1192;
    (t14776, t14777, t14779, t14781, t14782, t14784, t14785, t14787, t14788, t14791)
}
