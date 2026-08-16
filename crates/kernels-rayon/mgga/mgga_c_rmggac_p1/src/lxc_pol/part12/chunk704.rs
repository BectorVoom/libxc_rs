//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 704/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk704(t9091: f64, t9124: f64, t9126: f64, t9129: f64, t9148: f64, t9223: f64, t9225: f64, t9229: f64, t8328: f64, t8331: f64, t8334: f64, t8350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9619 = 0.19863479950205658386e-4_f64 * t9091;
    let t9636 = 0.1064114997332445985e-4_f64 * t9124;
    let t9646 = 0.2993560425465952141e-1_f64 * t9126;
    let t9647 = 0.5987120850931904282e-1_f64 * t9129;
    let t9653 = 0.1064114997332445985e-4_f64 * t9148;
    let t9670 = 0.1064114997332445985e-4_f64 * t9223;
    let t9671 = 0.8980681276397856423e-1_f64 * t9225;
    let t9672 = 0.5987120850931904282e-1_f64 * t9229;
    let t9716 = 0.19211284388664477842e-2_f64 * t8328;
    let t9717 = 0.81300399444200075504e-3_f64 * t8331;
    let t9718 = 0.81300399444200075504e-3_f64 * t8334;
    let t9729 = 0.30487649791575028314e-3_f64 * t8350;
    (t9619, t9636, t9646, t9647, t9653, t9670, t9671, t9672, t9716, t9717, t9718, t9729)
}
