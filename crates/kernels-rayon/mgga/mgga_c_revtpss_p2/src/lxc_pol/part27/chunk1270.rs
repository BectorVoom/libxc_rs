//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1270/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1270(t1358: f64, t2439: f64, t7274: f64, t785: f64, t26064: f64, t3920: f64, t1444: f64, t4004: f64, t213: f64, t225: f64, t25921: f64, t25930: f64, t25931: f64, t25933: f64, t25934: f64, t25961: f64, t27868: f64, t27980: f64, t46422: f64, t561: f64, t94574: f64, t94694: f64, t94700: f64, t94703: f64, t94705: f64, t94714: f64, t94716: f64, t94721: f64, t94726: f64, t94729: f64) -> f64 {
    let t94733 = t2439 * t785 * t7274 * t1358;
    let t94735 = t26064 * t3920;
    let t94737 = t4004 * t1444;
    let t94744 = 0.32927245914677557992e-1_f64 * t94694 + t94700 - t94703 - 0.52041769129231196772e1_f64 * t94705 * t25934 + 0.65854491829355115987e0_f64 * t213 * t94574 * t225 * t561 + 0.26020884564615598386e1_f64 * t25921 * t25961 - 0.21951497276451705329e-1_f64 * t94714 - 0.52041769129231196772e1_f64 * t25930 * t94716 * t25933 - 0.26020884564615598386e1_f64 * t25930 * t25931 * t94721 - 0.34697458558045176417e-2_f64 * t94726 - 0.32927245914677557992e-1_f64 * t94729 - 0.19514881078765566038e-2_f64 * t94733 - 0.39029762157531132076e-1_f64 * t94735 + 0.52041769129231196772e1_f64 * t25930 * t27980 * t94737 - 0.26020884564615598386e1_f64 * t27868 * t27980 * t46422;
    t94744
}
