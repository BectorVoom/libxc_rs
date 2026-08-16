//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 594/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk594(t3158: f64, t1071: f64, t740: f64, t829: f64, t1030: f64, t104: f64, t1072: f64, t120: f64, t2630: f64, t2635: f64, t3061: f64, t3136: f64, t3139: f64, t3142: f64, t3145: f64, t3150: f64, t3153: f64, t3154: f64) -> (f64, f64, f64, f64) {
    let t3159 = 0.15538616723388920628e-3_f64 * t3158;
    let t3160 = t740 * t1071;
    let t3161 = t3160 * t829;
    let t3165 = -0.10082625e-4_f64 * t120 * t3136 + 0.7026e-2_f64 * t104 * t3139 + 0.50413125e-5_f64 * t120 * t3142 - 0.672175e-5_f64 * t120 * t3145 - 0.23911438650126355246e-1_f64 * t3061 * t2630 + 0.15538616723388920628e-3_f64 * t3150 * t2630 - t3153 - 0.23911438650126355246e-1_f64 * t3154 + 0.11955719325063177623e-1_f64 * t1030 * t2635 + t3159 + 0.20718155631185227504e-3_f64 * t3161 - 0.5179538907796306876e-4_f64 * t1072 * t2635;
    (t3159, t3160, t3161, t3165)
}
