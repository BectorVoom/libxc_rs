//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1001/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1001<F: Float>(t8028: F, t900: F, t2332: F, t3147: F, t7930: F, t6090: F, t6093: F, t6127: F, t7947: F, t7955: F, t378: F, t6180: F, t6183: F, t6249: F, t7950: F, t7959: F, t7961: F, t7967: F) -> (F, F, F, F, F) {
    let t8030 = F::cast_from(0.11696447245269292414e1_f64) * t8028 * t900;
    let t8034 = F::cast_from(0.11696447245269292414e1_f64) * t3147 * t2332;
    let t8038 = F::cast_from(0.18541666666666666667e-1_f64) * t7930;
    let t8040 = -t6127 + F::cast_from(0.24722222222222222222e-1_f64) * t6090 - F::cast_from(0.92708333333333333333e-2_f64) * t6093 + F::cast_from(0.12361111111111111111e-1_f64) * t7955 - t8038 + F::cast_from(0.278125e-1_f64) * t7947;
    let t8041 = t8040 * t378;
    let t8045 = F::cast_from(0.103295e1_f64) * t7930;
    let t8054 = -t6249 + F::cast_from(0.13772666666666666667e1_f64) * t6090 - F::cast_from(0.516475e0_f64) * t6093 - t8045 + F::cast_from(0.1549425e1_f64) * t7947 + F::cast_from(0.34731666666666666667e0_f64) * t7950 + F::cast_from(0.3529725e1_f64) * t7959 + F::cast_from(0.6311625e0_f64) * t7961 - F::cast_from(0.20839e0_f64) * t6180 - F::cast_from(0.20839e0_f64) * t6183 + F::cast_from(0.68863333333333333333e0_f64) * t7955 - F::cast_from(0.3529725e1_f64) * t7967;
    (t8030, t8034, t8040, t8041, t8054)
}
