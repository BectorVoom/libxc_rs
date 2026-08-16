//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 857/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk857(t2000: f64, t5935: f64, t2007: f64, t1996: f64, t5927: f64, t16515: f64, t16520: f64, t16522: f64, t16525: f64, t16527: f64, t16529: f64, t16537: f64, t16540: f64) -> f64 {
    let t16542 = t5935 * t2000;
    let t16544 = t5935 * t2007;
    let t16546 = t1996 * t5927;
    let t16548 = -t16515 - t16520 - t16522 + t16525 + t16527 + t16529 + t16537 + 0.43284165449459373508e0_f64 * t16540 + 0.12985249634837812052e1_f64 * t16542 + 0.43284165449459373508e0_f64 * t16544 + 0.12985249634837812052e1_f64 * t16546;
    t16548
}
