//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1117/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1117<F: Float>(t22233: F, t22293: F, t22296: F, t2196: F, t3030: F, t1171: F, t6141: F, t2256: F, t3080: F, t1189: F, t6312: F, t1235: F, t5722: F) -> (F, F, F, F, F, F, F, F) {
    let t22800 = F::cast_from(0.20659e1_f64) * t22233;
    let t22811 = F::cast_from(0.104195e1_f64) * t22293;
    let t22812 = F::cast_from(0.104195e1_f64) * t22296;
    let t22820 = t3030 * t2196;
    let t22823 = t1171 * t6141;
    let t22826 = t3080 * t2256;
    let t22829 = t1189 * t6312;
    let t22919 = t1235 * t5722;
    (t22800, t22811, t22812, t22820, t22823, t22826, t22829, t22919)
}
