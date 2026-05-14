//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 909/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk909<F: Float>(t33: F, t41154: F, t1518: F, t1936: F, t670: F, t7724: F, t32342: F, t575: F, t1464: F, t8602: F, t136: F, t32141: F, t10309: F, t2247: F, t6957: F, t84: F, t8440: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100981 = t41154 * t33;
    let t105823 = t1518 * t1936;
    let t108120 = t7724 * t670;
    let t119422 = t32342 * t575;
    let t119424 = t8602 * t1464;
    let t119443 = t32141 * t136;
    let t119444 = t10309 * t119443;
    let t119451 = t2247 * t6957 * t136;
    let t119456 = t2247 * t119443;
    let t119457 = t8440 * t84;
    (t100981, t105823, t108120, t119422, t119424, t119444, t119451, t119456, t119457)
}
