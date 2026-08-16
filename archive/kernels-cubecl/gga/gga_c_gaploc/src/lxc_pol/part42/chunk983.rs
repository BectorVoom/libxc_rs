//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 983/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk983<F: Float>(t12250: F, t14436: F, t169: F, t1841: F, t270: F, t2954: F, t299: F, t3487: F, t39347: F, t42933: F, t42936: F, t42939: F, t44711: F, t44716: F, t44719: F, t44723: F, t44726: F, t44731: F, t44735: F, t44740: F, t44744: F, t44748: F, t50182: F, t681: F, t706: F, t734: F) -> F {
    let t50338 = t44711 + F::cast_from(0.51270174867614828558e-2_f64) * t1841 * t39347 * t2954 - F::cast_from(0.17090058289204942852e-2_f64) * t1841 * t12250 * t3487 * t734 - t44716 + F::cast_from(0.76905262301422242837e-2_f64) * t681 * t14436 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t706 * t50182 * t169 * t299 + t44719 - t44723 + t44726 - F::cast_from(0.38452631150711121419e-2_f64) * t42933 - F::cast_from(0.38452631150711121419e-2_f64) * t42936 - F::cast_from(0.38452631150711121419e-2_f64) * t42939 + t44731 - t44735 - t44740 + t44744 + t44748;
    t50338
}
