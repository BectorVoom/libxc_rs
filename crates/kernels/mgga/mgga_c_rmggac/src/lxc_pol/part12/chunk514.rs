//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 514/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk514<F: Float>(t446: F, t5656: F, t1487: F, t998: F, t472: F, t5527: F, t1201: F, t1206: F, t1209: F, t1480: F, t1486: F, t1488: F, t1491: F, t206: F, t207: F, t470: F, t473: F, t5637: F, t5647: F, t5653: F, t600: F, t602: F) -> F {
    let t5657 = t5656 * t446;
    let t5660 = t1487 * t998;
    let t5663 = t472 * t5527;
    let t5666 = F::new(3.0) * t1201 * t602 - F::new(12.0) * t1206 * t600 + F::new(3.0) * t1209 * t600 + F::new(6.0) * t1480 * t473 + F::new(60.0) * t1486 * t5653 - F::new(24.0) * t1486 * t5657 - F::new(12.0) * t1486 * t5660 - F::new(24.0) * t1488 * t5647 + F::new(6.0) * t1491 * t470 + F::new(3.0) * t206 * t5663 - t207 * t5637;
    t5666
}
