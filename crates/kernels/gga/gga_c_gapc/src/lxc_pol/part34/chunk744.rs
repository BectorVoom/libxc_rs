//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 744/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk744<F: Float>(t647: F, t9306: F, t2998: F, t9305: F, t2993: F, t9282: F, t3001: F, t129: F, t5987: F, t2987: F, t197: F, t5799: F, t2986: F, t2003: F, t3039: F, t9273: F, t9279: F, t9284: F, t9289: F, t9292: F, t9295: F, t9299: F, t9302: F) -> (F, F, F, F, F, F, F) {
    let t9307 = t647 * t9306;
    let t9308 = t2998 * t9307;
    let t9309 = t9305 * t9308;
    let t9311 = t2993 * t9282;
    let t9312 = t9311 * t3001;
    let t9314 = t5987 * t129;
    let t9315 = t9314 * t2987;
    let t9317 = t197 * t5799;
    let t9318 = t2986 * t9317;
    let t9320 = t3039 * t2003;
    let t9322 = -0.45841236415607425908e-8 * t9273 - 0.50647778388733212873e-6 * t9279 - 0.24761136101158459626e-5 * t9284 + 0.16882592796244404291e-6 * t9289 + 0.33765185592488808582e-6 * t9292 - 0.10120768229166666667e-4 * t9295 + 0.50680539737635041234e-4 * t9299 + 0.69504740211613770836e-4 * t9302 - 0.16882592796244404291e-6 * t9309 - 0.12380568050579229813e-5 * t9312 + 0.17376185052903442709e-3 * t9315 + 0.17376185052903442709e-3 * t9318 + 0.11594181388521408695e-4 * t9320;
    (t9308, t9309, t9312, t9315, t9318, t9320, t9322)
}
