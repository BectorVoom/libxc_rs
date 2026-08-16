//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 797/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk797(t15902: f64, t1787: f64, t1775: f64, t4515: f64, t15913: f64, t8291: f64, t15927: f64, t15768: f64, t3134: f64, t15763: f64, t3127: f64, t15936: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16370 = t1787 * t15902;
    let t16373 = t1775 * t4515;
    let t16375 = t8291 * t15913;
    let t16378 = t1787 * t15927;
    let t16381 = t3134 * t15768;
    let t16384 = t3127 * t15763;
    let t16387 = t1787 * t15936;
    (t16370, t16373, t16375, t16378, t16381, t16384, t16387)
}
