//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2322/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2322(t23788: f64, t67128: f64, t16949: f64, t25891: f64, t25927: f64, t98102: f64, t5966: f64, t868: f64, t1649: f64, t4255: f64, t870: f64, t28248: f64, t83555: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100638 = t23788 * t67128;
    let t100641 = t25891 * t16949;
    let t100644 = t25927 * t98102;
    let t100646 = t5966 * t868;
    let t100651 = t870 * t1649 * t4255;
    let t100656 = t83555 * t28248;
    (t100638, t100641, t100644, t100646, t100651, t100656)
}
