//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1212/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1212<F: Float>(t3721: F, t1105: F, t13086: F, t1118: F, t1162: F, t12182: F, t13112: F, t13606: F, t20173: F, t2376: F, t2408: F, t2409: F, t3207: F, t335: F, t338: F, t34773: F, t34922: F, t353: F, t35889: F, t36323: F, t3733: F, t3742: F, t3780: F, t3917: F, t39689: F, t43788: F, t43790: F, t4386: F, t47071: F, t831: F, t859: F, t9820: F) -> (F, F) {
    let t49172 = t3721 * t3721;
    let t49178 = t1105 * t13086;
    let t49192 = -t34773 * t4386 * t353 * t1118 * t3780 / F::cast_from(4.0_f64) + t36323 * t12182 / F::cast_from(4.0_f64) + t34922 * t13112 / F::cast_from(6.0_f64) + t39689 * t12182 / F::cast_from(4.0_f64) - t34773 * t859 * t353 * t1162 * t3780 / F::cast_from(8.0_f64) - t47071 * t3733 / F::cast_from(32.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3917 * t9820 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t43788 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t43790 + t335 * t338 * t353 * t20173 * t49172 / F::cast_from(4.0_f64) + t3207 * t2409 * t831 * t49178 / F::cast_from(4.0_f64) + t2408 * t2409 * t35889 * t3742 / F::cast_from(4.0_f64) + t2408 * t2409 * t2376 * t13606 * t1105 / F::cast_from(12.0_f64);
    (t49178, t49192)
}
