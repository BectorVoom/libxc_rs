//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 771/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk771<F: Float>(t649: F, t9197: F, t197: F, t5479: F, t2986: F, t1018: F, t1875: F, t1877: F, t3096: F, t2990: F, t3088: F, t5803: F) -> (F, F, F, F, F, F) {
    let t9198 = t9197 * t649;
    let t9200 = t197 * t5479;
    let t9201 = t2986 * t9200;
    let t9203 = t1875 * t1018;
    let t9204 = t3096 * t1877;
    let t9205 = t9203 * t9204;
    let t9207 = t3088 * t2990;
    let t9209 = t197 * t5803;
    (t9198, t9201, t9203, t9205, t9207, t9209)
}
