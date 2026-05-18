//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 513/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk513<F: Float>(t1228: F, t1518: F, t1190: F, t1497: F, t205: F, t5474: F, t23: F, t470: F, t4388: F, t589: F, t1144: F, t1156: F, t1392: F) -> (F, F, F, F, F, F) {
    let t5633 = t1228 * t1518;
    let t5636 = F::new(0.12805126321218922714e0) * t1190 * t1497;
    let t5637 = t5474 * t205;
    let t5647 = t470 * t23;
    let t5652 = t4388 * t589;
    let t5653 = t5652 * t1144;
    let t5656 = t1156 * t1392;
    (t5633, t5636, t5637, t5647, t5653, t5656)
}
