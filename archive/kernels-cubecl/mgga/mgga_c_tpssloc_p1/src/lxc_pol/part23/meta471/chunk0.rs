//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1402/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1402<F: Float>(t11147: F, t75836: F, t136: F, t3297: F, t11153: F, t1113: F, t1089: F, t75912: F, t1088: F, t123: F) -> (F, F, F, F, F, F, F) {
    let t77973 = t11147 * t75836;
    let t77975 = t136 * t3297 * t77973;
    let t77977 = t11153 * t75836;
    let t77979 = t136 * t1113 * t77977;
    let t77981 = t1089 * t75912;
    let t77983 = t136 * t1113 * t77981;
    let t77989 = t123 * t1088 * t77977;
    (t77973, t77975, t77977, t77979, t77981, t77983, t77989)
}
