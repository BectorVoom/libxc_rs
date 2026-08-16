//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1312/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1312<F: Float>(t185: F, t707: F, t75912: F, t58984: F, t46433: F, t46439: F, t1409: F, t4194: F, t67469: F, t59013: F, t12939: F, t16716: F, t5398: F) -> (F, F, F, F, F, F, F) {
    let t76024 = F::cast_from(4.0_f64) * t707 * t185 * t75912;
    let t76025 = F::cast_from(0.14649157844805236043e-2_f64) * t58984;
    let t76026 = F::cast_from(0.22787578869697033845e-2_f64) * t46433;
    let t76027 = F::cast_from(4.0_f64) * t46439;
    let t76030 = F::cast_from(48.0_f64) * t4194 * t67469 * t1409;
    let t76031 = F::cast_from(72.0_f64) * t59013;
    let t76034 = F::cast_from(144.0_f64) * t12939 * t16716 * t5398;
    (t76024, t76025, t76026, t76027, t76030, t76031, t76034)
}
