//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1346/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1346<F: Float>(t1113: F, t13776: F, t20898: F, t3975: F, t36889: F, t2417: F, t3972: F, t51548: F, t824: F, t1115: F, t1193: F, t13911: F, t13925: F, t14577: F, t22134: F, t26604: F, t29775: F, t335: F, t338: F, t4002: F, t50876: F, t51947: F, t54664: F, t54667: F, t54675: F, t54682: F, t54690: F, t8629: F, t892: F, t9201: F) -> F {
    let t54694 = t13776 * t3975 * t1113 * t20898;
    let t54697 = t13776 * t3975 * t36889;
    let t54702 = t3972 * t51548 * t1113 * t824 * t2417;
    let t54704 = -t1115 * t51947 / F::cast_from(48.0_f64) + t54664 / F::cast_from(24.0_f64) + t54667 - t335 * t338 * t9201 * t1193 / F::cast_from(96.0_f64) + t8629 * t50876 / F::cast_from(48.0_f64) - t54675 / F::cast_from(24.0_f64) + t29775 * t13911 / F::cast_from(24.0_f64) + t26604 * t13925 / F::cast_from(48.0_f64) - t54682 - t335 * t338 * t892 * t14577 / F::cast_from(48.0_f64) - t22134 * t4002 / F::cast_from(96.0_f64) + t54690 / F::cast_from(384.0_f64) - t54694 / F::cast_from(384.0_f64) - t54697 / F::cast_from(192.0_f64) + t54702 / F::cast_from(768.0_f64);
    t54704
}
