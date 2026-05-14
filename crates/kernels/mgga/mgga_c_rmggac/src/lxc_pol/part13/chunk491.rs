//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 491/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk491<F: Float>(t205: F, t5474: F, t23: F, t470: F, t4388: F, t589: F, t1144: F, t1156: F, t1392: F, t446: F, t1487: F, t998: F, t472: F, t5527: F, t1201: F, t1206: F, t1209: F, t1480: F, t1486: F, t1488: F, t1491: F, t206: F, t207: F, t473: F, t600: F, t602: F) -> (F,) {
    let t5637 = t5474 * t205;
    let t5647 = t470 * t23;
    let t5652 = t4388 * t589;
    let t5653 = t5652 * t1144;
    let t5656 = t1156 * t1392;
    let t5657 = t5656 * t446;
    let t5660 = t1487 * t998;
    let t5663 = t472 * t5527;
    let t5666 = 3.0 * t1201 * t602 - 12.0 * t1206 * t600 + 3.0 * t1209 * t600 + 6.0 * t1480 * t473 + 60.0 * t1486 * t5653 - 24.0 * t1486 * t5657 - 12.0 * t1486 * t5660 - 24.0 * t1488 * t5647 + 6.0 * t1491 * t470 + 3.0 * t206 * t5663 - t207 * t5637;
    (t5666,)
}
