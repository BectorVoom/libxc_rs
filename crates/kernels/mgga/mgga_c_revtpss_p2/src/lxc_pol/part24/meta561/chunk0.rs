//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1685/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1685<F: Float>(t3011: F, t3014: F, t88351: F, t981: F, t6392: F, t6244: F, t6258: F, t42013: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F) -> (F, F, F, F) {
    let t88607 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t3011 * t88351 * t3014;
    let t88628 = t6392 * t6392;
    let t88646 = t6244 * t6258;
    let t88660 = F::cast_from(0.22222222222222222222e-1_f64) * t77559 - F::cast_from(0.66666666666666666668e-1_f64) * t77561 + F::cast_from(0.12345679012345679012e-1_f64) * t77499 - F::cast_from(0.14814814814814814815e-1_f64) * t63453 + F::cast_from(0.44444444444444444445e-1_f64) * t63459 + t42013 + F::new(0.2e0) * t88085 - F::new(0.3e0) * t88089 + F::cast_from(0.50000000000000000001e-1_f64) * t88093 + F::cast_from(0.66666666666666666668e-1_f64) * t88097 - F::cast_from(0.22222222222222222222e-1_f64) * t63464;
    (t88607, t88628, t88646, t88660)
}
