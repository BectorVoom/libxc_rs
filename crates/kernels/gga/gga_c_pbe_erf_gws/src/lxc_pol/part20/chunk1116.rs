//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1116/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1116<F: Float>(t13776: F, t3975: F, t44206: F, t44196: F, t1192: F, t35889: F, t829: F, t830: F, t1114: F, t332: F, t9847: F, t14138: F, t1105: F, t1133: F, t13798: F, t50956: F) -> (F, F, F, F, F) {
    let t56077 = t13776 * t3975 * t44206;
    let t56080 = t13776 * t3975 * t44196;
    let t56082 = t35889 * t1192;
    let t56084 = t829 * t830 * t56082;
    let t56092 = t1114 * t9847 * t332;
    let t56093 = t56092 * t14138;
    let t56098 = t13776 * t50956 * t1105 * t1133 * t13798;
    (t56077, t56080, t56084, t56093, t56098)
}
