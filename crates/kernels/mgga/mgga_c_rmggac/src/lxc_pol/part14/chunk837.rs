//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 837/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk837<F: Float>(t270: F, t38843: F, t7349: F, t7351: F, t2019: F, t2339: F, t7926: F, t35594: F, t35608: F, t40136: F, t40139: F, t40143: F, t40149: F, t40154: F, t40159: F, t40164: F, t40172: F, t40177: F, t40182: F, t40185: F, t40188: F, t40191: F, t40194: F) -> (F,) {
    let t40198 = t7349 * t7351 * t38843 * t270;
    let t40201 = t2019 * t7926 * t2339;
    let t40203 = -0.10227998120342003148e-1 * t40136 + 0.20455996240684006296e-1 * t40139 - 0.1064114997332445985e-4 * t40143 - 0.212822999466489197e-4 * t40149 - 0.17025839957319135759e-4 * t40154 + 0.51077519871957407277e-4 * t40159 - 0.51077519871957407277e-4 * t40164 + 0.39914139006212695213e-1 * t35594 - 0.25538759935978703639e-4 * t40172 + 0.25538759935978703638e-4 * t40177 + 0.85129199786595678796e-5 * t40182 - 0.13637330827122670864e0 * t40185 + 0.40911992481368012592e0 * t40188 + 0.81823984962736025184e-1 * t40191 + 0.20455996240684006296e-1 * t40194 - 0.43368970657079495312e-4 * t40198 + 0.81300399444200075504e-3 * t40201 + t35608;
    (t40203,)
}
