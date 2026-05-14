//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 915/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk915<F: Float>(t1268: F, t174: F, t4537: F, t1216: F, t4605: F, t4623: F, t470: F, t1399: F, t4801: F, t1322: F, t4619: F, t4800: F, t1: F, t467: F, t4778: F, t388: F, t405: F, t837: F) -> (F, F, F, F, F, F, F) {
    let t18435 = 0.71233333333333333333e-1 * t174 * t1268 * t4537;
    let t18439 = 0.62336721237753107879e3 * t470 * t4605 * t1216 * t4623;
    let t18440 = t1399 * t4801;
    let t18441 = 0.2077890707925103596e3 * t18440;
    let t18442 = t1322 * t4619;
    let t18445 = 0.69263023597503453196e2 * t470 * t4800 * t18442;
    let t18447 = t4778 * t1 * t467;
    let t18448 = 0.73246220147012639764e-3 * t18447;
    let t18452 = 0.22161481481481481481e0 * t174 * t837 * t388 * t405;
    (t18435, t18439, t18441, t18442, t18445, t18448, t18452)
}
