//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 876/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk876(t34918: f64, t586: f64, t1369: f64, t28: f64, t32962: f64, t9073: f64, t920: f64, t446: f64, t1017: f64, t32967: f64, t89: f64, t5778: f64, t6615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34919 = t586 * t34918;
    let t34921 = t1369 * t28 * t34919;
    let t34924 = t9073 * t32962 * t920;
    let t34925 = t446 * t34924;
    let t34927 = t32967 * t1017;
    let t34928 = t28 * t34927;
    let t34929 = t89 * t34928;
    let t34931 = t5778 * t6615;
    (t34919, t34921, t34924, t34925, t34927, t34929, t34931)
}
