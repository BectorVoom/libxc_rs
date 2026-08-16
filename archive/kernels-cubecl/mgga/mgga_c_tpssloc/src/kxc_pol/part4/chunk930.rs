//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 930/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk930<F: Float>(t225: F, t4143: F, t4145: F, t1496: F, t9541: F, t2427: F, t4101: F, t2528: F, t4199: F, t2663: F, t4211: F, t2535: F) -> (F, F, F, F, F, F, F) {
    let t13053 = t4143 * t225;
    let t13065 = t4145 * t225;
    let t13087 = t9541 * t1496;
    let t13105 = F::cast_from(8.0_f64) * t2427 * t4101;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    (t13053, t13065, t13087, t13105, t13107, t13109, t13113)
}
