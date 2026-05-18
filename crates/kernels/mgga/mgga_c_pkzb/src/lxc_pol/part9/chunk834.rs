//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 834/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk834<F: Float>(t5955: F, t6012: F, t6010: F, t2019: F, t785: F, t306: F, t759: F, t2009: F, t2030: F, t2970: F, t5718: F, t2111: F, t751: F) -> (F, F, F, F, F, F, F, F) {
    let t6013 = t6012 * t5955;
    let t6014 = t6010 * t6013;
    let t6017 = t2019 * t785;
    let t6021 = t2019 * t306 * t759;
    let t6022 = t2030 * t2009;
    let t6023 = t2970 * t6022;
    let t6026 = t5718 * t306;
    let t6027 = t6012 * t2030;
    let t6028 = t6010 * t6027;
    let t6031 = t751 * t2111;
    (t6014, t6017, t6021, t6022, t6023, t6026, t6028, t6031)
}
