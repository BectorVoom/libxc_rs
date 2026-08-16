//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 676/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk676<F: Float>(t1376: F, t2: F, t464: F, t1520: F, t1531: F, t386: F, t518: F, t85: F, t462: F, t1510: F, t406: F, t1512: F, t410: F) -> (F, F, F, F, F) {
    let t5011 = t1376 * t2;
    let t5012 = t5011 * t464;
    let t5015 = t1520 * t1531;
    let t5018 = t386 * t518 * t85;
    let t5019 = t462 * t5018;
    let t5020 = F::cast_from(0.56968947174242584612e-3_f64) * t5019;
    let t5021 = t406 * t1510;
    let t5027 = t410 * t1512;
    (t5012, t5015, t5020, t5021, t5027)
}
