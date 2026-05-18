//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 773/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk773<F: Float>(t1997: F, t36520: F, t7335: F, t7927: F, t3924: F, t504: F, t2185: F, t7407: F, t507: F, t8629: F, t124: F, t338: F) -> (F, F, F, F, F, F) {
    let t36521 = t36520 * t1997;
    let t36527 = t7335 * t7927;
    let t36596 = t504 * t3924;
    let t36612 = t7407 * t2185;
    let t36629 = t507 * t8629;
    let t36632 = t124 * t338;
    (t36521, t36527, t36596, t36612, t36629, t36632)
}
