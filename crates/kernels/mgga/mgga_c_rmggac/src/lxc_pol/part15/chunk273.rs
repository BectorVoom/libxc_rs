//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 273/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk273<F: Float>(t128: F, t1614: F, t326: F, t1544: F, t305: F, t1547: F, t118: F, t1358: F, t321: F, t551: F) -> (F, F, F, F, F) {
    let t1615 = t128 * t1614;
    let t1616 = t326 * t1615;
    let t1618 = t305 * t1544;
    let t1620 = t326 * t1547;
    let t1622 = t118 * t1358;
    let t1624 = t551 * t321;
    (t1616, t1618, t1620, t1622, t1624)
}
