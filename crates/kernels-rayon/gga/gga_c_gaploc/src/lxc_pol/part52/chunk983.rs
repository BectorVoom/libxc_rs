//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 983/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk983(t12250: f64, t14436: f64, t169: f64, t1841: f64, t270: f64, t2954: f64, t299: f64, t3487: f64, t39347: f64, t42933: f64, t42936: f64, t42939: f64, t44711: f64, t44716: f64, t44719: f64, t44723: f64, t44726: f64, t44731: f64, t44735: f64, t44740: f64, t44744: f64, t44748: f64, t50182: f64, t681: f64, t706: f64, t734: f64) -> f64 {
    let t50338 = t44711 + 0.51270174867614828558e-2_f64 * t1841 * t39347 * t2954 - 0.17090058289204942852e-2_f64 * t1841 * t12250 * t3487 * t734 - t44716 + 0.76905262301422242837e-2_f64 * t681 * t14436 + 0.76905262301422242837e-2_f64 * t270 * t706 * t50182 * t169 * t299 + t44719 - t44723 + t44726 - 0.38452631150711121419e-2_f64 * t42933 - 0.38452631150711121419e-2_f64 * t42936 - 0.38452631150711121419e-2_f64 * t42939 + t44731 - t44735 - t44740 + t44744 + t44748;
    t50338
}
