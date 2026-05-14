//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 662/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk662<F: Float>(t10058: F, t1953: F, t702: F, t72: F, t2435: F, t5928: F, t1737: F, t699: F, t1364: F, t2448: F, t623: F, t570: F, t9540: F, t9523: F, t10120: F, t10124: F, t10135: F, t10137: F, t10141: F, t10151: F, t10154: F, t10156: F, t10158: F, t10162: F, t10164: F, t10309: F, t118: F, t5148: F, t5266: F, t8242: F, t8243: F, t8911: F, t8913: F, t8917: F) -> (F, F, F, F, F, F, F, F) {
    let t10378 = 0.5454932330849068346e-1 * t10058;
    let t10379 = t1953 * t702;
    let t10380 = t72 * t10379;
    let t10381 = t5928 * t2435;
    let t10382 = 0.79828278012425390428e-1 * t10381;
    let t10387 = t699 * t1737;
    let t10388 = t1364 * t10387;
    let t10389 = 0.23948483403727617128e0 * t10388;
    let t10390 = t623 * t2448;
    let t10391 = 0.39914139006212695214e-1 * t10390;
    let t10395 = t9540 * t570;
    let t10403 = t9523 * t570;
    let t10414 = 0.10909864661698136692e0 * t8911 - 0.1454648621559751559e0 * t8913 - 0.36366215538993788974e-1 * t8917 + 0.23948483403727617128e0 * t5266 * t10395 + 0.2727466165424534173e-1 * t10120 + 0.68186654135613354325e-2 * t10124 - 0.35922725105591425692e0 * t10135 - 0.11974241701863808564e0 * t10137 + 0.35922725105591425692e0 * t10141 - 0.23948483403727617128e0 * t5148 * t10403 + t8242 - t8243 + 0.5987120850931904282e-1 * t10151 - 0.2993560425465952141e-1 * t10154 + 0.5987120850931904282e-1 * t10156 - 0.8980681276397856423e-1 * t10158 - 0.20455996240684006298e-1 * t10162 + 0.11974241701863808564e0 * t118 * t10309 - 0.13637330827122670865e0 * t10164;
    (t10378, t10379, t10380, t10382, t10387, t10389, t10391, t10414)
}
