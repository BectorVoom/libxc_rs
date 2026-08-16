//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1078/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1078<F: Float>(t33353: F, t33356: F, t33358: F, t33360: F, t33364: F, t33369: F, t33371: F, t33375: F, t33377: F, t33380: F, t33383: F, t11320: F, t2629: F, t933: F) -> (F, F) {
    let t33385 = F::cast_from(0.45289771048911752714e-7_f64) * t33353 + F::cast_from(0.52838066223730378166e-7_f64) * t33356 + F::cast_from(0.11594181388521408695e-4_f64) * t33358 - F::cast_from(0.11594181388521408695e-4_f64) * t33360 + F::cast_from(0.35848176214430067278e-9_f64) * t33364 + F::cast_from(0.47342907336462418838e-4_f64) * t33369 + F::cast_from(0.10821235962619981449e-3_f64) * t33371 - F::cast_from(0.33816362383187442026e-5_f64) * t33375 - F::cast_from(0.27173862629347051628e-6_f64) * t33377 - F::cast_from(0.90579542097823505428e-7_f64) * t33380 - F::cast_from(0.16908181191593721013e-5_f64) * t33383;
    let t33387 = t933 * t11320 * t2629;
    (t33385, t33387)
}
