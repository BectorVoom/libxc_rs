//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1346/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1346(t1113: f64, t13776: f64, t20898: f64, t3975: f64, t36889: f64, t2417: f64, t3972: f64, t51548: f64, t824: f64, t1115: f64, t1193: f64, t13911: f64, t13925: f64, t14577: f64, t22134: f64, t26604: f64, t29775: f64, t335: f64, t338: f64, t4002: f64, t50876: f64, t51947: f64, t54664: f64, t54667: f64, t54675: f64, t54682: f64, t54690: f64, t8629: f64, t892: f64, t9201: f64) -> f64 {
    let t54694 = t13776 * t3975 * t1113 * t20898;
    let t54697 = t13776 * t3975 * t36889;
    let t54702 = t3972 * t51548 * t1113 * t824 * t2417;
    let t54704 = -t1115 * t51947 / 48.0_f64 + t54664 / 24.0_f64 + t54667 - t335 * t338 * t9201 * t1193 / 96.0_f64 + t8629 * t50876 / 48.0_f64 - t54675 / 24.0_f64 + t29775 * t13911 / 24.0_f64 + t26604 * t13925 / 48.0_f64 - t54682 - t335 * t338 * t892 * t14577 / 48.0_f64 - t22134 * t4002 / 96.0_f64 + t54690 / 384.0_f64 - t54694 / 384.0_f64 - t54697 / 192.0_f64 + t54702 / 768.0_f64;
    t54704
}
