//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1052/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1052(t3025: f64, t3255: f64, t4752: f64, t33232: f64, t787: f64, t9824: f64, t41405: f64, t41408: f64, t43586: f64, t7584: f64, t7585: f64, t10012: f64, t2684: f64, t2925: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43989 = 0.7150097990370085334e0_f64 * t3025 * t4752 * t3255;
    let t43991 = t787 * t33232 * t9824;
    let t43993 = 0.20854452471912748891e0_f64 * t41405;
    let t43994 = 0.19171462976960374838e0_f64 * t41408;
    let t43997 = t7584 * t7585 * t43586;
    let t44001 = t2684 * t9438 * t10012 * t2925;
    (t43989, t43991, t43993, t43994, t43997, t44001)
}
