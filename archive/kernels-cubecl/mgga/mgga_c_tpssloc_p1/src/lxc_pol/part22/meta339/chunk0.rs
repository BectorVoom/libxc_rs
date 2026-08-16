//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1534/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1534<F: Float>(t40: F, t12943: F, t4101: F, t4205: F, t4202: F, t16558: F, t185: F, t707: F, t5392: F, t634: F, t5398: F, t75: F, t3966: F, t4104: F, t607: F, t767: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t16629 = F::cast_from(0.23392894490538584828e1_f64) * t12943;
    let t16630 = t4205 * t4101;
    let t16631 = F::cast_from(8.0_f64) * t16630;
    let t16633 = F::cast_from(8.0_f64) * t4205 * t4202;
    let t16634 = t185 * t16558;
    let t16636 = F::cast_from(4.0_f64) * t707 * t16634;
    let t16637 = t634 * t5392;
    let t16642 = t75 * t5398;
    let t16648 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16637 * t607 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4104 * t3966 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16642 * t607 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t16558);
    (t16629, t16630, t16631, t16633, t16634, t16636, t16637, t16648)
}
