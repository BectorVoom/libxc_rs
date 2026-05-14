//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 804/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk804<F: Float>(t504: F, t9827: F, t2282: F, t9483: F, t2732: F, t6241: F) -> (F, F, F, F) {
    let t9828 = t9827 * t504;
    let t9829 = t9483 * t2282;
    let t9830 = t6241 * t2732;
    let t9831 = t2732 * t2282;
    (t9828, t9829, t9830, t9831)
}
