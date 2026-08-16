//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 942/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk942(t5011: f64, t511: f64, t2136: f64, t270: f64, t38843: f64, t7349: f64, t7351: f64, t2019: f64, t2339: f64, t7926: f64, t35594: f64, t35608: f64, t40136: f64, t40139: f64, t40143: f64, t40149: f64, t40154: f64, t40159: f64, t40164: f64, t40172: f64, t40177: f64, t40182: f64, t40185: f64, t40188: f64, t40191: f64) -> f64 {
    let t40193 = t5011 * t511;
    let t40194 = t40193 * t2136;
    let t40198 = t7349 * t7351 * t38843 * t270;
    let t40201 = t2019 * t7926 * t2339;
    let t40203 = -0.10227998120342003148e-1_f64 * t40136 + 0.20455996240684006296e-1_f64 * t40139 - 0.1064114997332445985e-4_f64 * t40143 - 0.212822999466489197e-4_f64 * t40149 - 0.17025839957319135759e-4_f64 * t40154 + 0.51077519871957407277e-4_f64 * t40159 - 0.51077519871957407277e-4_f64 * t40164 + 0.39914139006212695213e-1_f64 * t35594 - 0.25538759935978703639e-4_f64 * t40172 + 0.25538759935978703638e-4_f64 * t40177 + 0.85129199786595678796e-5_f64 * t40182 - 0.13637330827122670864e0_f64 * t40185 + 0.40911992481368012592e0_f64 * t40188 + 0.81823984962736025184e-1_f64 * t40191 + 0.20455996240684006296e-1_f64 * t40194 - 0.43368970657079495312e-4_f64 * t40198 + 0.81300399444200075504e-3_f64 * t40201 + t35608;
    t40203
}
