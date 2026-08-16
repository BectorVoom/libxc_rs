//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1009/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1009(t1209: f64, t2297: f64, t1196: f64, t6290: f64, t2258: f64, t3136: f64, t889: f64, t2312: f64, t3139: f64, t2320: f64, t3135: f64, t1208: f64, t6233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
