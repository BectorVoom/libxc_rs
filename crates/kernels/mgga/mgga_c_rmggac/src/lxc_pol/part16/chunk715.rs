//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 715/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk715<F: Float>(t10390: F, t570: F, t9540: F, t9523: F, t10120: F, t10124: F, t10135: F, t10137: F, t10141: F, t10151: F, t10154: F, t10156: F, t10158: F, t10162: F, t10164: F, t10309: F, t118: F, t5148: F, t5266: F, t8242: F, t8243: F, t8911: F, t8913: F, t8917: F) -> (F, F) {
    let t10391 = F::cast_from(0.39914139006212695214e-1_f64) * t10390;
    let t10395 = t9540 * t570;
    let t10403 = t9523 * t570;
    let t10414 = F::cast_from(0.10909864661698136692e0_f64) * t8911 - F::cast_from(0.1454648621559751559e0_f64) * t8913 - F::cast_from(0.36366215538993788974e-1_f64) * t8917 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t10395 + F::cast_from(0.2727466165424534173e-1_f64) * t10120 + F::cast_from(0.68186654135613354325e-2_f64) * t10124 - F::cast_from(0.35922725105591425692e0_f64) * t10135 - F::cast_from(0.11974241701863808564e0_f64) * t10137 + F::cast_from(0.35922725105591425692e0_f64) * t10141 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t10403 + t8242 - t8243 + F::cast_from(0.5987120850931904282e-1_f64) * t10151 - F::cast_from(0.2993560425465952141e-1_f64) * t10154 + F::cast_from(0.5987120850931904282e-1_f64) * t10156 - F::cast_from(0.8980681276397856423e-1_f64) * t10158 - F::cast_from(0.20455996240684006298e-1_f64) * t10162 + F::cast_from(0.11974241701863808564e0_f64) * t118 * t10309 - F::cast_from(0.13637330827122670865e0_f64) * t10164;
    (t10391, t10414)
}
