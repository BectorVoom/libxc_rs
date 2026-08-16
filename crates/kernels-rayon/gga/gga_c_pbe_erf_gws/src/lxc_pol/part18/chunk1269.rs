//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1269/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1269(t3975: f64, t45096: f64, t51555: f64, t3861: f64, t3972: f64, t9520: f64, t13776: f64, t44206: f64, t44196: f64, t1192: f64, t35889: f64, t829: f64, t830: f64) -> (f64, f64, f64, f64, f64) {
    let t56070 = t51555 * t3975 * t45096;
    let t56074 = t3972 * t3975 * t3861 * t9520;
    let t56077 = t13776 * t3975 * t44206;
    let t56080 = t13776 * t3975 * t44196;
    let t56082 = t35889 * t1192;
    let t56084 = t829 * t830 * t56082;
    (t56070, t56074, t56077, t56080, t56084)
}
