//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1952/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1952(t25927: f64, t98102: f64, t5966: f64, t868: f64, t1649: f64, t4255: f64, t870: f64, t28248: f64, t83555: f64, t98030: f64, t23788: f64, t98011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100644 = t25927 * t98102;
    let t100646 = t5966 * t868;
    let t100651 = t870 * t1649 * t4255;
    let t100656 = t83555 * t28248;
    let t100659 = t25927 * t98030;
    let t100664 = t23788 * t98011;
    (t100644, t100646, t100651, t100656, t100659, t100664)
}
