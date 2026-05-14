//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 110/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk110<F: Float>(t228: F, t231: F, t234: F, t245: F) -> (F, F, F) {
    let t280 = 0.51785e1 * t231 + 0.905775e0 * t228 + 0.1100325e0 * t234 + 0.1241775e0 * t245;
    let t283 = 1.0 + 0.29608749977793437516e2 / t280;
    let t284 = f64::ln(t283);
    (t280, t283, t284)
}
