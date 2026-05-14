//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 868/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk868<F: Float>(t1546: F, t17501: F, t2060: F, t577: F, t1467: F, t4294: F, t12520: F, t492: F, t15973: F, t6028: F, t2051: F, t4307: F, t16751: F, t1548: F, t16622: F, t4288: F) -> (F, F, F, F, F, F, F) {
    let t17502 = t1546 * t17501;
    let t17504 = t577 * t2060;
    let t17505 = t1467 * t17504;
    let t17506 = t17505 * t4294;
    let t17508 = t12520 * t492;
    let t17509 = t6028 * t15973;
    let t17510 = t17508 * t17509;
    let t17512 = t2051 * t4307;
    let t17514 = t16751 * t577;
    let t17515 = t17514 * t1548;
    let t17517 = t16622 * t577;
    let t17518 = t17517 * t4288;
    (t17502, t17506, t17509, t17510, t17512, t17515, t17518)
}
