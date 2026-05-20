//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1562/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1562<F: Float>(t12331: F, t3391: F, t3399: F, t12322: F, t12346: F, t25273: F, t268: F, t404: F) -> (F, F, F) {
    let t43808 = t12331 * t3391 * t3399;
    let t43810 = t12346 * t12322;
    let t43813 = t268 * t25273 * t404;
    (t43808, t43810, t43813)
}
