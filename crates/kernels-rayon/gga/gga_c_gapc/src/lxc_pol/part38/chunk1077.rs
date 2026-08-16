//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1077/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1077(t33353: f64, t33356: f64, t33358: f64, t33360: f64, t33364: f64, t33369: f64, t33371: f64, t33375: f64, t33377: f64, t33380: f64, t33383: f64, t11320: f64, t2629: f64, t933: f64) -> (f64, f64) {
    let t33385 = 0.45289771048911752714e-7_f64 * t33353 + 0.52838066223730378166e-7_f64 * t33356 + 0.11594181388521408695e-4_f64 * t33358 - 0.11594181388521408695e-4_f64 * t33360 + 0.35848176214430067278e-9_f64 * t33364 + 0.47342907336462418838e-4_f64 * t33369 + 0.10821235962619981449e-3_f64 * t33371 - 0.33816362383187442026e-5_f64 * t33375 - 0.27173862629347051628e-6_f64 * t33377 - 0.90579542097823505428e-7_f64 * t33380 - 0.16908181191593721013e-5_f64 * t33383;
    let t33387 = t933 * t11320 * t2629;
    (t33385, t33387)
}
