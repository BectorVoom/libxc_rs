//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1225/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1225<F: Float>(t1548: F, t22691: F, t1929: F, t570: F, t5910: F, t2043: F, t5999: F, t1466: F, t7380: F, t1535: F, t1552: F, t7322: F, t1543: F, t7287: F, t17474: F, t5932: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t22692 = t22691 * t1548;
    let t22694 = t570 * t1929;
    let t22695 = t22694 * t5910;
    let t22697 = t5999 * t2043;
    let t22699 = t7380 * t1466;
    let t22700 = t22699 * sigma2;
    let t22701 = t22700 * t1535;
    let t22703 = t7322 * t1552;
    let t22705 = t1543 * t7287;
    let t22707 = t17474 * t5932;
    (t22692, t22695, t22697, t22701, t22703, t22705, t22707)
}
