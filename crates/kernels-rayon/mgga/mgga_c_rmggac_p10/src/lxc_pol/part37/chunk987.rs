//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 987/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk987(t76159: f64, t71863: f64, t71871: f64, t71892: f64, t76173: f64, t76175: f64, t76178: f64, t76186: f64, t76188: f64, t76190: f64, t36: f64, t9565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77850 = 0.40911992481368012595e-1_f64 * t76159;
    let t77851 = 0.18183107769496894486e-1_f64 * t71863;
    let t77852 = 0.36366215538993788972e-1_f64 * t71871;
    let t77853 = 0.27274661654245341729e-1_f64 * t71892;
    let t77860 = 0.20455996240684006296e-1_f64 * t76173;
    let t77863 = 0.40911992481368012592e-1_f64 * t76175;
    let t77864 = 0.20455996240684006296e-1_f64 * t76178;
    let t77868 = 0.20455996240684006298e-1_f64 * t76186;
    let t77869 = 0.2727466165424534173e-1_f64 * t76188;
    let t77870 = 0.13637330827122670865e-1_f64 * t76190;
    let t77871 = t9565 * t36;
    (t77850, t77851, t77852, t77853, t77860, t77863, t77864, t77868, t77869, t77870, t77871)
}
