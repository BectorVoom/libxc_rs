//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 285/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk285<F: Float>(t1475: F, t446: F, t221: F, t1439: F, t205: F, t206: F, t23: F, t1156: F, t589: F, t1392: F, t472: F, t207: F, t470: F, t473: F, t600: F, t602: F) -> (F, F, F, F, F, F, F) {
    let t1476 = t1475 * t446;
    let t1477 = t221 * t1476;
    let t1480 = t1439 * t205;
    let t1486 = t206 * t23;
    let t1487 = t1156 * t589;
    let t1488 = t1487 * t446;
    let t1491 = t472 * t1392;
    let t1494 = -t1480 * t207 - 12.0 * t1486 * t1488 + 3.0 * t1491 * t206 + 3.0 * t470 * t602 + 3.0 * t473 * t600;
    (t1477, t1480, t1486, t1487, t1488, t1491, t1494)
}
