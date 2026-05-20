//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2923/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923<F: Float>(t52091: F, t52092: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F) -> F {
    let t77797 = -F::new(4.0) / F::new(3.0) * t63338 + F::new(4.0) / F::new(9.0) * t63340 + F::new(10.0) / F::new(27.0) * t63342 + F::new(2.0) * t63361 - F::new(4.0) / F::new(3.0) * t63371 + t52091 - t52092 + t63447 / F::new(3.0) - F::new(8.0) / F::new(27.0) * t63453 + F::new(8.0) / F::new(9.0) * t63459 + F::new(2.0) / F::new(9.0) * t77559 - F::new(2.0) / F::new(3.0) * t77561 + F::new(40.0) / F::new(9.0) * t77566 - F::new(10.0) / F::new(9.0) * t77570 - F::new(80.0) / F::new(81.0) * t77575 - F::new(4.0) / F::new(9.0) * t63464 + F::new(2.0) / F::new(3.0) * t77581 - F::new(2.0) / F::new(9.0) * t77586 - F::new(8.0) * t77590 + F::new(4.0) * t77594;
    t77797
}
