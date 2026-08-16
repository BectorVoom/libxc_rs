//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1220/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1220(t1369: f64, t80866: f64, t22782: f64, t3777: f64, t22783: f64, t3876: f64, t80807: f64, t80810: f64, t80814: f64, t80817: f64, t80821: f64, t80826: f64, t80828: f64, t80831: f64, t80833: f64, t80837: f64, t80843: f64, t80848: f64, t80850: f64, t80857: f64, t80859: f64, t80861: f64, t80863: f64) -> f64 {
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    let t80870 = t80869 * t1369;
    let t80872 = t22783 * t3876;
    let t80874 = 0.10093189023535097714e-3_f64 * t80807 + t80810 / 1536.0_f64 + 0.60559134141210586281e-3_f64 * t80814 + t80817 / 64.0_f64 - 7.0_f64 / 96.0_f64 * t80821 - t80826 - 7.0_f64 / 16.0_f64 * t80828 - t80831 / 4.0_f64 + t80833 / 128.0_f64 + 0.3027956707060529314e-3_f64 * t80837 - 0.42391393898847410397e-2_f64 * t80843 - t80848 - t80850 / 128.0_f64 - 0.12111826828242117256e-2_f64 * t80857 - 35.0_f64 / 192.0_f64 * t80859 - 5.0_f64 / 64.0_f64 * t80861 + 5.0_f64 / 128.0_f64 * t80863 - 119.0_f64 / 576.0_f64 * t80867 + 7.0_f64 / 96.0_f64 * t80870 + 7.0_f64 / 192.0_f64 * t80872;
    t80874
}
