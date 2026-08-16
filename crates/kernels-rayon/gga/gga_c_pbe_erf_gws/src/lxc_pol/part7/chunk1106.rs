//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1106/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1106(t4459: f64, t6155: f64, t19561: f64, t816: f64, t825: f64, t2373: f64, t6745: f64, t2365: f64, t6158: f64, t6164: f64, t822: f64, t1452: f64, t19750: f64, t19772: f64, t19778: f64, t19791: f64, t2118: f64, t2358: f64, t2362: f64, t2382: f64, t2384: f64, t2392: f64, t3079: f64, t328: f64, t4385: f64, t6106: f64, t6145: f64, t6151: f64, t6160: f64, t8606: f64) -> (f64, f64) {
    let t19794 = t6155 * t4459;
    let t19803 = t19561 * t816;
    let t19804 = t19803 * t825;
    let t19808 = t6745 * t2373;
    let t19810 = t6158 * t2365;
    let t19812 = t822 * t19810 * t6164;
    let t19816 = t4385 * t19772 / 8.0_f64 - 7.0_f64 / 24.0_f64 * t19778 + t2382 * t2118 * t1452 * t328 * t3079 / 24.0_f64 + t2382 * t2118 * t19750 * t8606 / 8.0_f64 + t6160 * t19791 / 12.0_f64 + 7.0_f64 / 24.0_f64 * t19794 - t6106 * t2358 * t2362 / 32.0_f64 + t2384 * t6145 / 8.0_f64 + 3.0_f64 / 8.0_f64 * t2384 * t6151 + 11.0_f64 / 96.0_f64 * t2382 * t19804 * t6164 + 7.0_f64 / 12.0_f64 * t19808 - 7.0_f64 / 36.0_f64 * t19812 + 3.0_f64 / 8.0_f64 * t2392 * t6151;
    (t19803, t19816)
}
