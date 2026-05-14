//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 826/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk826<F: Float>(t2220: F, t238: F, t801: F, t2224: F, t2311: F, t835: F, t2310: F, t280: F) -> (F, F, F, F) {
    let t6619 = t238 * t801 * t2220;
    let t6622 = t238 * t801 * t2224;
    let t6636 = t835 * t2311;
    let t6640 = 1.0 / t2310 / t280;
    (t6619, t6622, t6636, t6640)
}
