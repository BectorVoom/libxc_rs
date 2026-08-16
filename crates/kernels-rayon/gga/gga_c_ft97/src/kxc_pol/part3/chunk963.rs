//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 963/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk963(t5446: f64, t8675: f64, t1268: f64, t2923: f64, t3746: f64, t4969: f64, t904: f64, t17744: f64, t4342: f64, t17780: f64, t4973: f64, t17727: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18877 = t8675 * t5446;
    let t18880 = t2923 * t3746 * t1268;
    let t18884 = t2923 * t4969 * t904;
    let t18887 = t4342 * t17744;
    let t18889 = t4342 * t17780;
    let t18893 = t2923 * t4973 * t904;
    let t18896 = t4342 * t17727;
    (t18877, t18880, t18884, t18887, t18889, t18893, t18896)
}
