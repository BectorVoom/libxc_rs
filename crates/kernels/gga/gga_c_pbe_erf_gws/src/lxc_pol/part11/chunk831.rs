//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 831/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk831<F: Float>(t13187: F, t2409: F, t831: F, t1115: F, t11349: F, t11368: F, t12234: F, t13142: F, t13174: F, t13184: F, t2503: F, t3047: F, t3052: F, t3207: F, t3733: F, t3913: F, t833: F, t8747: F, t9815: F, t9849: F, t9912: F, t9953: F, t9956: F, t9962: F) -> (F, F) {
    let t13189 = t2409 * t831 * t13187;
    let t13201 = F::new(7.0) / F::new(24.0) * t9912 - F::new(7.0) / F::new(24.0) * t9953 + t13142 * t833 / F::new(96.0) - F::new(7.0) / F::new(96.0) * t9956 - F::new(7.0) / F::new(96.0) * t9962 + F::new(35.0) / F::new(144.0) * t8747 + t13174 * t833 / F::new(96.0) + t1115 * t12234 / F::new(32.0) + t3913 * t2503 / F::new(32.0) - F::new(7.0) / F::new(96.0) * t11349 - F::new(3.0) / F::new(16.0) * t3207 * t13184 + F::new(3.0) / F::new(16.0) * t3207 * t13189 + F::new(7.0) / F::new(96.0) * t11368 - t9815 * t3733 / F::new(48.0) - t9849 * t3733 / F::new(48.0) - t3913 * t3052 / F::new(16.0) - t3913 * t3047 / F::new(32.0);
    (t13189, t13201)
}
