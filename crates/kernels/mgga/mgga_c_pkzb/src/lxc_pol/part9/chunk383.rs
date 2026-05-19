//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 383/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk383<F: Float>(t1511: F, t557: F, t470: F, t71: F, t57: F, t490: F) -> (F, F, F, F, F, F) {
    let t1512 = t1511 * t557;
    let t1513 = F::cast_from(0.11696447245269292414e1_f64) * t1512;
    let t1514 = t470 * t71;
    let t1515 = F::new(1.0) / t1514;
    let t1516 = t57 * t1515;
    let t1517 = t490 * t490;
    (t1512, t1513, t1514, t1515, t1516, t1517)
}
