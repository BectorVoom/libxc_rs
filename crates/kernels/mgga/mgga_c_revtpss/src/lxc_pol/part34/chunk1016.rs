//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1016/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1016<F: Float>(t239: F, t25981: F, t820: F, t240: F, t7262: F, t2482: F, t27: F, t25273: F, t533: F, t816: F, t540: F, t7021: F, t1389: F, t7269: F, t2736: F, t2689: F, t7256: F) -> (F, F, F, F, F, F, F, F) {
    let t25983 = t820 * t25981 * t239;
    let t25986 = t7262 * t240;
    let t25997 = t2482 * t7262 * t27;
    let t26002 = t25273 * t533 * t816;
    let t26003 = 35.0 / 432.0 * t26002;
    let t26004 = t7021 * t540;
    let t26009 = t7269 * t1389;
    let t26010 = t2736 * t26009;
    let t26011 = 0.50820002809285328225e-5 * t26010;
    let t26012 = t2689 * t7256;
    (t25983, t25986, t25997, t26003, t26004, t26009, t26011, t26012)
}
