//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 110/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk110<F: Float>(t228: F, t231: F, t234: F, t245: F) -> (F, F, F) {
    let t280 = F::cast_from(0.51785e1_f64) * t231 + F::cast_from(0.905775e0_f64) * t228 + F::cast_from(0.1100325e0_f64) * t234 + F::cast_from(0.1241775e0_f64) * t245;
    let t283 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t280;
    let t284 = F::ln(t283);
    (t280, t283, t284)
}
