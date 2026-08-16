//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 836/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk836<F: Float>(t20368: F, t44386: F, t1358: F, t23915: F, t161: F, t37573: F, t2339: F, t172: F, t3338: F, t550: F, t13396: F, t2299: F, t488: F) -> (F, F, F, F, F, F) {
    let t44387 = t20368 * t44386;
    let t44390 = F::cast_from(0.18970004423784099732e-1_f64) * t1358 * t23915 * t44387;
    let t44391 = t37573 * t161;
    let t44394 = F::cast_from(0.94850022118920498663e-2_f64) * t1358 * t44391 * t2339;
    let t44395 = t172 * t3338;
    let t44396 = t550 * t44395;
    let t44403 = F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t2299 * t13396 * t488;
    (t44387, t44390, t44394, t44395, t44396, t44403)
}
