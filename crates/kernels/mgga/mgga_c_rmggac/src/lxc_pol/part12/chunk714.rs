//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 714/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk714<F: Float>(t5542: F, t7546: F, t674: F, t7269: F, t7508: F, t2084: F, t2145: F, t27: F, t866: F, t1347: F, t2153: F, t1987: F, t7939: F, t2185: F, t7407: F, t7411: F) -> (F, F, F, F, F, F, F) {
    let t36541 = t7546 * t5542;
    let t36542 = t36541 * t674;
    let t36590 = t7508 * t7269;
    let t36594 = t2145 * t27 * t2084 * t866;
    let t36601 = t1347 * t2153;
    let t36610 = t7939 * t1987;
    let t36612 = t7407 * t2185;
    let t36613 = t36612 * t7411;
    (t36541, t36542, t36590, t36594, t36601, t36610, t36613)
}
