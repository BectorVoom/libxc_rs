//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 974/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk974(t11847: f64, t147: f64, t311: f64, t11579: f64, t919: f64, t128: f64, t2211: f64, t2545: f64, t2578: f64, t3297: f64, t3761: f64, t869: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11848 = t11847 * t147;
    let t11849 = t311 * t11848;
    let t11850 = t11579 * t919;
    let t11851 = t11849 * t11850;
    let t11853 = t2211 * t128;
    let t11854 = t2545 * t11853;
    let t11855 = t2578 * t11854;
    let t11858 = t869 * t3761 * t3297;
    (t11848, t11849, t11850, t11851, t11853, t11854, t11855, t11858)
}
