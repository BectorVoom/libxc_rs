//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1341/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1341<F: Float>(t26751: F, t415: F, t9469: F, t1451: F, t8163: F, t33557: F, t5975: F, t25967: F, t2722: F, t1333: F, t34786: F, t1413: F, t8162: F, t1441: F, t110016: F, t8073: F) -> (F, F, F, F, F, F, F) {
    let t119560 = t415 * t9469 * t26751;
    let t119563 = t415 * t8163 * t1451;
    let t119566 = t415 * t33557 * t5975;
    let t119569 = t415 * t25967 * t2722;
    let t119573 = t1333 * t34786;
    let t119575 = t8162 * t1413;
    let t119577 = t415 * t119575 * t1441;
    let t119580 = t415 * t110016 * t8073;
    (t119560, t119563, t119566, t119569, t119573, t119577, t119580)
}
