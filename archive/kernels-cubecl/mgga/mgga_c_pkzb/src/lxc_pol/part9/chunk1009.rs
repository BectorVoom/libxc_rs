//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1009/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1009<F: Float>(t1209: F, t2297: F, t1196: F, t6290: F, t2258: F, t3136: F, t889: F, t2312: F, t3139: F, t2320: F, t3135: F, t1208: F, t6233: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8150 = t1209 * t2297;
    let t8153 = t1196 * t6290;
    let t8154 = t8153 * t2258;
    let t8161 = t3136 * t889;
    let t8164 = t1209 * t2312;
    let t8167 = t3139 * t2297;
    let t8170 = t3135 * t2320;
    let t8171 = t8170 * t889;
    let t8174 = t3139 * t2312;
    let t8177 = t1208 * t6233;
    (t8150, t8153, t8154, t8161, t8164, t8167, t8170, t8171, t8174, t8177)
}
