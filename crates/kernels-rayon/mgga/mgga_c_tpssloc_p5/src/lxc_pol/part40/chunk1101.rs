//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1101/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1101(t225: f64, t5849: f64, t1603: f64, t4657: f64, t1634: f64, t4693: f64, t3174: f64, t5851: f64, t17183: f64, t977: f64, t17178: f64, t2979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17575 = t5849 * t225;
    let t17579 = t1603 * t4657;
    let t17582 = t1634 * t4693;
    let t17583 = t3174 * t17582;
    let t17588 = t5851 * t225;
    let t17593 = t977 * t17183;
    let t17596 = t2979 * t17178;
    (t17575, t17579, t17583, t17588, t17593, t17596)
}
