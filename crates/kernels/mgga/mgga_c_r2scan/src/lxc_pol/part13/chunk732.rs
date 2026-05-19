//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 732/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk732<F: Float>(t5720: F, t61: F, t1871: F, t584: F, t608: F, t4741: F, t5309: F, t5312: F, t5315: F, t171: F, t718: F, t226: F, t5456: F) -> (F, F, F, F, F, F) {
    let t5853 = F::cast_from(0.65061487801810439052e-1_f64) * t61 * t5720;
    let t5855 = t584 * t608 * t1871;
    let t5860 = F::cast_from(0.32547666666666666667e-1_f64) * t4741;
    let t5861 = -F::cast_from(0.14816666666666666667e-1_f64) * t5309 + F::cast_from(0.9877777777777777778e-2_f64) * t5312 - F::cast_from(0.46096296296296296297e-1_f64) * t5315 - t5860;
    let t5864 = F::new(0.571528e-1) * t584 * t171 * t5861;
    let t5865 = t61 * t718;
    let t5866 = t226 * t5456;
    (t5853, t5855, t5860, t5864, t5865, t5866)
}
