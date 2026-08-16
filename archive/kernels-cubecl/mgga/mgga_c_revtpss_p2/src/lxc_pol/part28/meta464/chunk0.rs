//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1767/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1767<F: Float>(t25227: F, t2664: F, t2661: F, t2670: F, t7033: F, t2482: F, t27: F, t7043: F) -> (F, F, F, F, F) {
    let t25228 = t25227 * t2664;
    let t25229 = t2661 * t25228;
    let t25230 = F::cast_from(0.28582678745379824648e-4_f64) * t25229;
    let t25231 = t7033 * t2670;
    let t25232 = F::cast_from(0.27104001498285508387e-3_f64) * t25231;
    let t25234 = t2482 * t7043 * t27;
    (t25228, t25229, t25230, t25232, t25234)
}
