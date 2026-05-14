//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 901/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk901<F: Float>(t574: F, t8165: F, t1632: F, t2654: F, t551: F, t1592: F, t1584: F, t2620: F, t1567: F, t978: F, t255: F, t571: F, t2086: F, t980: F, t2627: F, t6518: F) -> (F, F, F, F, F, F, F, F) {
    let t8167 = 0.23115257973478049502e0 * t574 * t8165;
    let t8176 = t551 * t1632 * t2654;
    let t8178 = 0.69345773920434148506e0 * t1592 * t8176;
    let t8189 = 0.23115257973478049502e0 * t1584 * t2620;
    let t8196 = t1567 * t978;
    let t8198 = t571 * t8196 * t255;
    let t8201 = t980 * t2086;
    let t8224 = 0.76830240467580968652e0 * t6518 * t2627;
    (t8167, t8176, t8178, t8189, t8196, t8198, t8201, t8224)
}
