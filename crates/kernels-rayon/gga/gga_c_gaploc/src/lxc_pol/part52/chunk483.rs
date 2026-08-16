//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 483/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk483(t313: f64, t8637: f64, t1022: f64, t701: f64, t739: f64, t8502: f64, t2610: f64, t7290: f64, t321: f64, t107: f64, t787: f64, t1858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8638 = t313 * t8637;
    let t8669 = t1022 * t701;
    let t8670 = t739 * t8669;
    let t8682 = t739 * t8502;
    let t8756 = t2610 * t8669;
    let t8769 = t7290 * t8502;
    let t8773 = t321 * t1022;
    let t8774 = t8773 * t107;
    let t8775 = t787 * t8774;
    let t8788 = t1858 * t1022;
    (t8638, t8669, t8670, t8682, t8756, t8769, t8773, t8774, t8775, t8788)
}
