//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 788/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk788<F: Float>(t2084: F, t2145: F, t27: F, t866: F, t1987: F, t7939: F, t2185: F, t7407: F, t7411: F, t507: F, t8629: F, t124: F, t338: F) -> (F, F, F, F, F) {
    let t36594 = t2145 * t27 * t2084 * t866;
    let t36610 = t7939 * t1987;
    let t36612 = t7407 * t2185;
    let t36613 = t36612 * t7411;
    let t36629 = t507 * t8629;
    let t36632 = t124 * t338;
    (t36594, t36610, t36613, t36629, t36632)
}
