//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1156/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1156<F: Float>(t1891: F, t1894: F, t21062: F, t1647: F, t188: F, t5389: F, t5447: F, t5390: F, t644: F, t1898: F, t5448: F, t1399: F, t1810: F, t1957: F, t2006: F, t2009: F, t207: F, t21420: F, t390: F, t5286: F, t5296: F, t5504: F, t5508: F, t5512: F, t5513: F, t5589: F, t5636: F, t5640: F, t682: F) -> (F, F, F, F, F) {
    let t21923 = 0.1551780387578202009e4 * t1891 * t1894 * t21062;
    let t21927 = 0.37242729301876848216e5 * t5447 * t188 * t5389 * t1647;
    let t21930 = 0.6207121550312808036e4 * t1891 * t644 * t5390;
    let t21958 = 0.23158050478117250496e4 * t1891 * t1898 * t5448;
    let t21959 = -t21923 + t21927 - t21930 + 0.6207121550312808036e4 * t2006 * t2009 * t21420 - 0.54794666666666666664e0 * t1399 * t5504 - 0.123288e1 * t390 * t5636 * t5512 - 0.27397333333333333333e0 * t1399 * t5508 + 0.82191999999999999998e0 * t1399 * t5513 + 0.164384e1 * t390 * t5640 * t207 * t5589 - 0.39036892681086263432e0 * t390 * t5296 + 0.26024595120724175621e0 * t1399 * t1810 + 0.52049190241448351242e0 * t390 * t5286 - 96.0 * t1957 * t682 * t5589 - t21958;
    (t21923, t21927, t21930, t21958, t21959)
}
