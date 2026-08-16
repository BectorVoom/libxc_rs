//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 904/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk904(t24850: f64, t7327: f64, t24574: f64, t7365: f64, t1235: f64, t477: f64, t225: f64, t7349: f64, t7288: f64, t7306: f64, t3640: f64, t7394: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24851 = t7327 * t24850;
    let t24856 = t24574 * t7365;
    let t24858 = t477 * t1235;
    let t24880 = t7349 * t225;
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24905 = t7394 * t3640;
    (t24851, t24856, t24858, t24880, t24891, t24893, t24905)
}
