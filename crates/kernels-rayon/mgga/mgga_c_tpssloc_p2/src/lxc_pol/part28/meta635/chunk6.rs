//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2017/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2017(t90956: f64, t90961: f64, t90963: f64, t90970: f64, t90980: f64, t90983: f64, t90987: f64, t90993: f64, t1338: f64, t27051: f64, t12267: f64, t1336: f64, t1352: f64, t1825: f64, t24128: f64, t27074: f64, t27103: f64, t3777: f64, t3856: f64, t5234: f64, t5344: f64, t7932: f64, t81115: f64, t81125: f64, t84581: f64, t90968: f64) -> f64 {
    let t93588 = 0.76763589786250567036e-1_f64 * t90956;
    let t93589 = 0.3289868133696452873e-1_f64 * t90961;
    let t93590 = 0.15352717957250113407e0_f64 * t90963;
    let t93592 = 0.76763589786250567036e-1_f64 * t90970;
    let t93595 = 0.16449340668482264365e-1_f64 * t90980;
    let t93599 = 0.16449340668482264365e-1_f64 * t90983;
    let t93600 = 0.16449340668482264365e-1_f64 * t90987;
    let t93605 = 0.16449340668482264365e-1_f64 * t90993;
    let t93607 = t1338 * t27051;
    let t93612 = t93588 - t93589 - t93590 + 0.3289868133696452873e-1_f64 * t90968 + t93592 - 2.0_f64 * t3777 * t27103 + t93595 - t12267 * t7932 - t5344 * t27074 * t3856 + t93599 - t93600 - t1336 * t84581 * t1825 + 2.0_f64 * t5234 * t24128 - t93605 + 0.82246703342411321825e-2_f64 * t81115 - 2.0_f64 * t1336 * t93607 * t1352 + 0.82246703342411321825e-2_f64 * t81125;
    t93612
}
