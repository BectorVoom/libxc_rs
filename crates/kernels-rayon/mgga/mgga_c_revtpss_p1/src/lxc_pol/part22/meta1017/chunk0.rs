//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3516/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3516(t11773: f64, t4954: f64, t1011: f64, t6284: f64, t697: f64, t19900: f64, t3241: f64, t11883: f64, t12004: f64, t16223: f64, t19707: f64, t19917: f64, t42740: f64, t42745: f64, t42756: f64, t54198: f64, t54222: f64, t54259: f64, t54857: f64, t6285: f64, t6331: f64) -> (f64, f64) {
    let t66542 = t4954 * t11773;
    let t66547 = t1011 * t697 * t6284;
    let t66551 = t3241 * t19900;
    let t66558 = -5.0_f64 / 243.0_f64 * t42740 - t42745 - 11.0_f64 / 486.0_f64 * t42756 - 0.60976381323476959249e-2_f64 * t54198 - 0.96545937095505185476e-2_f64 * t12004 * t6331 + 0.95275595817932748826e-3_f64 * t66542 * t16223 - 0.20325460441158986416e-2_f64 * t54222 + t66547 / 648.0_f64 - 11.0_f64 / 162.0_f64 * t11883 * t6285 + t66551 / 81.0_f64 - t3241 * t19917 / 54.0_f64 + 0.11433071498151929859e-2_f64 * t54857 * t19707 - 0.60976381323476959249e-2_f64 * t54259;
    (t66542, t66558)
}
