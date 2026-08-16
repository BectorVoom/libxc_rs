//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 817/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk817(t2986: f64, t9317: f64, t2003: f64, t3039: f64, t9273: f64, t9279: f64, t9284: f64, t9289: f64, t9292: f64, t9295: f64, t9299: f64, t9302: f64, t9309: f64, t9312: f64, t9315: f64) -> (f64, f64, f64) {
    let t9318 = t2986 * t9317;
    let t9320 = t3039 * t2003;
    let t9322 = -0.45841236415607425908e-8_f64 * t9273 - 0.50647778388733212873e-6_f64 * t9279 - 0.24761136101158459626e-5_f64 * t9284 + 0.16882592796244404291e-6_f64 * t9289 + 0.33765185592488808582e-6_f64 * t9292 - 0.10120768229166666667e-4_f64 * t9295 + 0.50680539737635041234e-4_f64 * t9299 + 0.69504740211613770836e-4_f64 * t9302 - 0.16882592796244404291e-6_f64 * t9309 - 0.12380568050579229813e-5_f64 * t9312 + 0.17376185052903442709e-3_f64 * t9315 + 0.17376185052903442709e-3_f64 * t9318 + 0.11594181388521408695e-4_f64 * t9320;
    (t9318, t9320, t9322)
}
