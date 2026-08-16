//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 653/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk653(t5152: f64, t639: f64, t1692: f64, t617: f64, t2677: f64, t1620: f64, t1726: f64, t633: f64, t4359: f64, t220: f64, t186: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5154 = 4.0_f64 / 9.0_f64 * t639 * t5152;
    let t5155 = t1692 * t617;
    let t5156 = t2677 * t5155;
    let t5158 = 8.0_f64 / 9.0_f64 * t1620 * t5156;
    let t5160 = 2.0_f64 / 5.0_f64 * t633 * t1726;
    let t5162 = -3.0_f64 * t4359;
    let t5163 = t220 * t5162;
    let t5164 = t186 * t5163;
    let t5166 = 4.0_f64 / 15.0_f64 * t616 * t5164;
    (t5154, t5155, t5156, t5158, t5160, t5162, t5163, t5164, t5166)
}
