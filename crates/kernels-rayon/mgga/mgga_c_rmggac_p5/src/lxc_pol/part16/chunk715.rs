//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 715/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk715(t10390: f64, t570: f64, t9540: f64, t9523: f64, t10120: f64, t10124: f64, t10135: f64, t10137: f64, t10141: f64, t10151: f64, t10154: f64, t10156: f64, t10158: f64, t10162: f64, t10164: f64, t10309: f64, t118: f64, t5148: f64, t5266: f64, t8242: f64, t8243: f64, t8911: f64, t8913: f64, t8917: f64) -> (f64, f64) {
    let t10391 = 0.39914139006212695214e-1_f64 * t10390;
    let t10395 = t9540 * t570;
    let t10403 = t9523 * t570;
    let t10414 = 0.10909864661698136692e0_f64 * t8911 - 0.1454648621559751559e0_f64 * t8913 - 0.36366215538993788974e-1_f64 * t8917 + 0.23948483403727617128e0_f64 * t5266 * t10395 + 0.2727466165424534173e-1_f64 * t10120 + 0.68186654135613354325e-2_f64 * t10124 - 0.35922725105591425692e0_f64 * t10135 - 0.11974241701863808564e0_f64 * t10137 + 0.35922725105591425692e0_f64 * t10141 - 0.23948483403727617128e0_f64 * t5148 * t10403 + t8242 - t8243 + 0.5987120850931904282e-1_f64 * t10151 - 0.2993560425465952141e-1_f64 * t10154 + 0.5987120850931904282e-1_f64 * t10156 - 0.8980681276397856423e-1_f64 * t10158 - 0.20455996240684006298e-1_f64 * t10162 + 0.11974241701863808564e0_f64 * t118 * t10309 - 0.13637330827122670865e0_f64 * t10164;
    (t10391, t10414)
}
