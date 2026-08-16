//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1156/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1156(t10696: f64, t863: f64, t10698: f64, t10536: f64, t1882: f64, t10688: f64, t10712: f64, t2749: f64, t2770: f64, t2811: f64, t8232: f64, t10516: f64, t10666: f64, t10704: f64, t10799: f64, t15229: f64, t1901: f64, t2739: f64, t2862: f64, t2867: f64, t2894: f64, t296: f64, t319: f64, t43409: f64, t43513: f64, t446: f64, t824: f64, t840: f64, t871: f64) -> (f64, f64, f64) {
    let t44351 = t863 * t10696;
    let t44352 = t44351 * t10698;
    let t44360 = t1882 * t10536;
    let t44362 = t10688 * t10712;
    let t44369 = t2770 * t2749;
    let t44381 = t8232 * t2811;
    let t44387 = 4.0_f64 / 3.0_f64 * t446 * t840 * t871 * t10799 * t824 - 8.0_f64 * t446 * t296 * t44352 - 8.0_f64 * t446 * t2862 * t2749 * t10516 + 4.0_f64 / 9.0_f64 * t44360 + 8.0_f64 * t446 * t296 * t44362 - 8.0_f64 / 3.0_f64 * t1901 * t15229 * t43409 - 8.0_f64 / 3.0_f64 * t1901 * t44369 * t10704 - 2.0_f64 * t446 * t840 * t2894 * t2739 + 2.0_f64 * t446 * t2862 * t319 * t43513 - 8.0_f64 / 9.0_f64 * t44381 + 4.0_f64 * t446 * t840 * t10666 * t2867;
    (t44352, t44362, t44387)
}
