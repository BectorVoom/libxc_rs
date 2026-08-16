//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1302/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1302(t13776: f64, t36889: f64, t3975: f64, t1113: f64, t2417: f64, t3972: f64, t51548: f64, t824: f64, t13781: f64, t13782: f64, t3038: f64, t13792: f64, t8716: f64) -> (f64, f64, f64, f64) {
    let t54697 = t13776 * t3975 * t36889;
    let t54702 = t3972 * t51548 * t1113 * t824 * t2417;
    let t54707 = t3972 * t13781 * t3038 * t13782;
    let t54714 = t13792 * t8716;
    (t54697, t54702, t54707, t54714)
}
