//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 465/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk465(t3042: f64, t41: f64, t447: f64, t445: f64, t1394: f64, t429: f64, t431: f64, t3812: f64, t213: f64, t442: f64, t1390: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3845 = t3042 * t41;
    let t3846 = t3845 * t447;
    let t3848 = 0.16804375e-4_f64 * t445 * t3846;
    let t3851 = 0.8197e-2_f64 * t429 * t1394 * t431;
    let t3852 = 0.23911438650126355246e-1_f64 * t3812;
    let t3857 = t213 * t442;
    let t3858 = 0.15538616723388920628e-3_f64 * t3857;
    let t3859 = t967 * t1390;
    (t3845, t3848, t3851, t3852, t3858, t3859)
}
