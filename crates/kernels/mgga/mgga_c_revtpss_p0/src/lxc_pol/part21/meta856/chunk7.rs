//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3254/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3254<F: Float>(t5: F, t10301: F, t10309: F, t10310: F, t10313: F, t13272: F, t13286: F, t13289: F, t13420: F, t2247: F, t2248: F, t2315: F, t4178: F, t4241: F, t45931: F, t45933: F, t45941: F, t45944: F, t45952: F, t45958: F, t60214: F, t60215: F, t60216: F, t60217: F, t60218: F, t60221: F, t60224: F, t60496: F, t644: F, t91: F) -> F {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t60498 = piecewise3::<F>(t8, F::cast_from(0.0_f64), (-t60214 + t45931 - t45933 + t60215 - t60216 + t45941 - t45944 + t60217 - t60218 + t45952) * t91 + F::cast_from(60.0_f64) * t60221 * t2248 - F::cast_from(120.0_f64) * t60224 * t10310 + F::cast_from(60.0_f64) * t13272 * t10313 + F::cast_from(60.0_f64) * t45958 * t4178 + F::cast_from(120.0_f64) * t10301 * t13286 + F::cast_from(60.0_f64) * t10301 * t13289 - F::cast_from(360.0_f64) * t10309 * t4241 * t2248 + F::cast_from(60.0_f64) * t2247 * t13420 * t644 + F::cast_from(60.0_f64) * t2247 * t4241 * t2315 + t60496);
    t60498
}
