//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 966/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk966(t5468: f64, t668: f64, t505: f64, t2923: f64, t10839: f64, t12143: f64, t18877: f64, t18880: f64, t18884: f64, t18887: f64, t18889: f64, t18893: f64, t18896: f64, t18900: f64, t18902: f64, t18905: f64, t18908: f64, t18911: f64, t18914: f64, t18919: f64, t18923: f64, t18928: f64, t18931: f64, t18936: f64, t2265: f64, t631: f64) -> f64 {
    let t18938 = t5468 * t668;
    let t18939 = t18938 * t505;
    let t18940 = t2923 * t18939;
    let t18943 = 4.0_f64 / 9.0_f64 * t18877 + 4.0_f64 / 3.0_f64 * t12143 * t18880 + 2.0_f64 / 3.0_f64 * t2265 * t18884 + t2265 * t18887 + 4.0_f64 / 3.0_f64 * t12143 * t18889 - t2265 * t18893 / 3.0_f64 - t2265 * t18896 / 3.0_f64 + 5.0_f64 / 9.0_f64 * t10839 + 2.0_f64 / 9.0_f64 * t18900 - t18902 / 9.0_f64 - t2265 * t18905 / 9.0_f64 - t2265 * t18908 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t2265 * t18911 - 2.0_f64 / 9.0_f64 * t12143 * t18914 - 3.0_f64 / 2.0_f64 * t631 * t18919 + t631 * t18923 / 6.0_f64 + 6.0_f64 * t631 * t18928 + t2265 * t18931 / 18.0_f64 + t2265 * t18936 - t2265 * t18940 / 3.0_f64;
    t18943
}
