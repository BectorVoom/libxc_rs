//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 723/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk723<F: Float>(t325: F, t5058: F, t117: F, t26125: F, t108: F, t5751: F, t128: F, t25640: F, t1652: F, t333: F, t305: F, t4616: F) -> (F, F, F, F, F, F) {
    let t26857 = t5058 * t325;
    let t27006 = t26125 * t117;
    let t27036 = t5751 * t108;
    let t27041 = t25640 * t128;
    let t27044 = t1652 * t333;
    let t27048 = t305 * t4616;
    (t26857, t27006, t27036, t27041, t27044, t27048)
}
