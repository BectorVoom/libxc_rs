//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1207/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1207<F: Float>(t15121: F, t804: F, t15389: F, t321: F, t14831: F, t30104: F, t12275: F, t14825: F, t15128: F, t11889: F, t13756: F, t14149: F, t14383: F, t14821: F, t15124: F, t3928: F, t3944: F, t3946: F, t4062: F, t52774: F, t52823: F, t52853: F, t52855: F, t52860: F) -> (F,) {
    let t57799 = t804 * t15121;
    let t57801 = t321 * t15389;
    let t57803 = t30104 * t14831;
    let t57809 = t12275 * t14825;
    let t57817 = t804 * t15128;
    let t57819 = 12.0 * t11889 * t13756 * t3944 - 6.0 * t14149 * t15124 * t3946 - t14149 * t3928 * t4062 - 6.0 * t14383 * t14821 * t3946 - 6.0 * t14821 * t14825 * t3946 + 12.0 * t52774 * t57803 - 12.0 * t52823 * t57809 + t52853 + t52855 - t52860 + 6.0 * t57799 + 2.0 * t57801 + 3.0 * t57817;
    (t57819,)
}
