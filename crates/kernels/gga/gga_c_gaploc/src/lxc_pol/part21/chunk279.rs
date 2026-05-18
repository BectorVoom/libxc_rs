//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 279/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk279<F: Float>(t1112: F, t1114: F, t1116: F, t1099: F, t1101: F, t1105: F, t1108: F, t14: F, t344: F, t389: F, t31: F, t4: F, t98: F) -> (F, F) {
    let t1118 = -F::new(0.44044444444444444445e-2) * t1112 + F::new(0.88088888888888888889e-2) * t1114 + F::new(0.55033333333333333333e-2) * t1116;
    let t1121 = -t1099 * t1101 / F::new(18.0) - t1105 * t344 / F::new(6.0) + t389 * t1108 / F::new(9.0) + t14 * t1118 / F::new(2.0);
    let t1126 = F::new(0.14764770444444444444e-2) * t4 * t98 * t31;
    (t1121, t1126)
}
