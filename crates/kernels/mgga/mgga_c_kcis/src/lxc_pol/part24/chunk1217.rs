//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1217/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1217<F: Float>(t389: F, t69377: F, t20197: F, t26930: F, t13181: F, t1817: F, t20200: F, t26891: F, t29056: F, t19715: F, t95463: F, t20169: F) -> (F, F, F, F, F, F, F) {
    let t99878 = t69377 * t389;
    let t99880 = t26930 * t20197;
    let t99882 = t13181 * t1817;
    let t99884 = t26930 * t20200;
    let t99886 = t26891 * t29056;
    let t99888 = t95463 * t19715;
    let t99890 = t26930 * t20169;
    (t99878, t99880, t99882, t99884, t99886, t99888, t99890)
}
