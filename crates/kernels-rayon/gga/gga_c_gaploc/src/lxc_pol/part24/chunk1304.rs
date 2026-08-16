//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1304/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1304(t33386: f64, t10883: f64, t7416: f64, t1980: f64, t8788: f64, t9824: f64, t22424: f64, t3500: f64, t2975: f64, t6134: f64, t7372: f64, t1: f64, t32364: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33387 = 0.38342925953920749676e0_f64 * t33386;
    let t33388 = t7416 * t10883;
    let t33389 = 0.85206502119823888168e-1_f64 * t33388;
    let t33391 = t1980 * t8788 * t9824;
    let t33392 = 0.29792074959875355558e-1_f64 * t33391;
    let t33393 = t22424 * t3500;
    let t33394 = 0.19171462976960374838e0_f64 * t33393;
    let t33396 = t6134 * t2975 * t7372;
    let t33397 = 0.29792074959875355558e-1_f64 * t33396;
    let t33399 = t787 * t32364 * t1;
    (t33387, t33389, t33392, t33394, t33397, t33399)
}
