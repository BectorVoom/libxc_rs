//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2124/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2124<F: Float>(t10701: F, t1543: F, t10810: F, t1561: F, t47705: F, t47707: F, t48096: F, t47730: F, t48155: F, t48157: F, t2929: F, t4446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t49274 = t1543 * t10701;
    let t49285 = t1561 * t10810;
    let t49304 = F::cast_from(0.13772666666666666666e1_f64) * t47705;
    let t49306 = F::cast_from(0.45908888888888888888e0_f64) * t47707;
    let t49317 = F::cast_from(0.34731666666666666667e0_f64) * t48096;
    let t49322 = F::cast_from(0.68863333333333333332e0_f64) * t47730;
    let t49378 = F::cast_from(0.69463333333333333334e0_f64) * t48155;
    let t49379 = F::cast_from(0.11577222222222222222e0_f64) * t48157;
    let t49411 = t4446 * t2929;
    (t49274, t49285, t49304, t49306, t49317, t49322, t49378, t49379, t49411)
}
