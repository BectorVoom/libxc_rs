//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2584/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2584<F: Float>(t45619: F, t58919: F, t3666: F, t5390: F, t43766: F, t44361: F, t45608: F, t45786: F, t12984: F, t5323: F, t17500: F, t372: F) -> (F, F, F, F, F, F, F) {
    let t58920 = t45619 * t58919;
    let t58927 = t3666 * t5390;
    let t58983 = t44361 * t43766;
    let t59001 = t45608 * t58919;
    let t59011 = t45786 * t58919;
    let t59040 = t5323 * t12984;
    let t59041 = F::cast_from(0.7622047665434619906e-3_f64) * t59040;
    let t59062 = t372 * t17500;
    (t58920, t58927, t58983, t59001, t59011, t59041, t59062)
}
