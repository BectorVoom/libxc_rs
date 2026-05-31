//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 492/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk492<F: Float>(t3158: F, t1071: F, t740: F, t829: F, t1030: F, t104: F, t1072: F, t120: F, t2630: F, t2635: F, t3061: F, t3136: F, t3139: F, t3142: F, t3145: F, t3150: F, t3153: F, t3154: F) -> (F, F, F, F) {
    let t3159 = F::cast_from(0.15538616723388920628e-3_f64) * t3158;
    let t3160 = t740 * t1071;
    let t3161 = t3160 * t829;
    let t3165 = -F::cast_from(0.10082625e-4_f64) * t120 * t3136 + F::cast_from(0.7026e-2_f64) * t104 * t3139 + F::cast_from(0.50413125e-5_f64) * t120 * t3142 - F::cast_from(0.672175e-5_f64) * t120 * t3145 - F::cast_from(0.23911438650126355246e-1_f64) * t3061 * t2630 + F::cast_from(0.15538616723388920628e-3_f64) * t3150 * t2630 - t3153 - F::cast_from(0.23911438650126355246e-1_f64) * t3154 + F::cast_from(0.11955719325063177623e-1_f64) * t1030 * t2635 + t3159 + F::cast_from(0.20718155631185227504e-3_f64) * t3161 - F::cast_from(0.5179538907796306876e-4_f64) * t1072 * t2635;
    (t3159, t3160, t3161, t3165)
}
