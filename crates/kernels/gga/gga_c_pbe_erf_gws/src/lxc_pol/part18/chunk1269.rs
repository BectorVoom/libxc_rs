//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1269/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1269<F: Float>(t3975: F, t45096: F, t51555: F, t3861: F, t3972: F, t9520: F, t13776: F, t44206: F, t44196: F, t1192: F, t35889: F, t829: F, t830: F) -> (F, F, F, F, F) {
    let t56070 = t51555 * t3975 * t45096;
    let t56074 = t3972 * t3975 * t3861 * t9520;
    let t56077 = t13776 * t3975 * t44206;
    let t56080 = t13776 * t3975 * t44196;
    let t56082 = t35889 * t1192;
    let t56084 = t829 * t830 * t56082;
    (t56070, t56074, t56077, t56080, t56084)
}
