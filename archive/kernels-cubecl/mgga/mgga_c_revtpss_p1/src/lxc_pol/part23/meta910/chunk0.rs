//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2923/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923<F: Float>(t52091: F, t52092: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F) -> F {
    let t77797 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63338 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t63340 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t63342 + F::cast_from(2.0_f64) * t63361 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63371 + t52091 - t52092 + t63447 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t63453 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t63459 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t77559 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t77561 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t77566 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t77570 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t77575 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t63464 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t77581 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t77586 - F::cast_from(8.0_f64) * t77590 + F::cast_from(4.0_f64) * t77594;
    t77797
}
