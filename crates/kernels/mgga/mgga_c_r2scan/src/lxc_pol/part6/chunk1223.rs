//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1223/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1223<F: Float>(t2061: F, t22602: F, t1375: F, t6027: F, t6029: F, t1654: F, t2056: F, t6028: F, t2060: F, t2062: F, t4958: F, t6006: F, t6007: F, t607: F, t1783: F, t2055: F) -> (F, F, F, F, F, F, F) {
    let t22603 = t2061 * t22602;
    let t22606 = t6027 * t1375 * t6029;
    let t22608 = t1654 * t2056;
    let t22609 = t6028 * t22608;
    let t22612 = t2060 * t4958 * t2062;
    let t22616 = t6006 * t607 * t6007;
    let t22619 = t2055 * t1783 * t2056;
    (t22603, t22606, t22608, t22609, t22612, t22616, t22619)
}
