//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1320/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1320(t1955: f64, t22964: f64, t108188: f64, t1882: f64, t2030: f64, t22971: f64, t25924: f64, t27837: f64, t30021: f64, t30055: f64, t30071: f64, t543: f64, t6895: f64, t6918: f64, t7279: f64, t7295: f64, t7296: f64, t7301: f64, t7910: f64, t7930: f64, t94602: f64, t94608: f64, t97792: f64, t97795: f64, t97800: f64, t97810: f64, t97815: f64) -> f64 {
    let t114485 = t1955 * t22964;
    let t114513 = -0.4336814094102599731e0_f64 * t114485 * t2030 + 0.52041769129231196772e1_f64 * t27837 * t30021 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t30055 * t1882 * t543 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t7910 * t6895 - 0.13010442282307799193e1_f64 * t30071 * t7930 - 0.86736281882051994623e-1_f64 * t108188 + t94602 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t7910 * t6918 + 0.21951497276451705329e-1_f64 * t97792 + 0.19514881078765566038e-2_f64 * t97795 - 0.68549505033305214441e-2_f64 * t97800 - t94608 + 0.34697458558045176417e-2_f64 * t97810 + 0.13709901006661042888e-1_f64 * t97815 + 0.39512695097613069591e1_f64 * t7279 * t22971;
    t114513
}
