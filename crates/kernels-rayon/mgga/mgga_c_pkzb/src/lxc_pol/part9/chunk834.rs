//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 834/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk834(t5955: f64, t6012: f64, t6010: f64, t2019: f64, t785: f64, t306: f64, t759: f64, t2009: f64, t2030: f64, t2970: f64, t5718: f64, t2111: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
