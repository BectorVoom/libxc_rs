//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 999/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk999<F: Float>(t35315: F, t4987: F, t7647: F, t1980: F, t34487: F, t7476: F, t2314: F, t31258: F, t1982: F, t568: F, t13299: F, t31057: F, t35288: F) -> (F, F, F, F, F, F) {
    let t35316 = F::new(0.64311027177104605458e-2) * t35315;
    let t35317 = t7647 * t4987;
    let t35318 = F::new(0.17149607247227894789e-2) * t35317;
    let t35348 = t1980 * t7476 * t34487;
    let t35349 = F::new(0.7145669686344956162e-3) * t35348;
    let t35359 = t31258 * t2314;
    let t35364 = t568 * t1982;
    let t35379 = t31057 * t13299 * t35288;
    (t35316, t35318, t35349, t35359, t35364, t35379)
}
