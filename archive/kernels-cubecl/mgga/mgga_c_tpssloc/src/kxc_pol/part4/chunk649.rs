//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 649/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk649<F: Float>(t4189: F, t4264: F, t218: F, t1520: F, t225: F, t1527: F, t865: F, t2718: F, t2627: F, t68: F, t226: F, t1509: F, t252: F) -> (F, F, F, F, F, F, F) {
    let t4265 = t4189 + t4264;
    let t4266 = t218 * t4265;
    let t4268 = t1520 * t225;
    let t4272 = t1527 * t865;
    let t4273 = t2718 * t4272;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4282 = t252 * t1509;
    (t4265, t4266, t4268, t4273, t4280, t4281, t4282)
}
