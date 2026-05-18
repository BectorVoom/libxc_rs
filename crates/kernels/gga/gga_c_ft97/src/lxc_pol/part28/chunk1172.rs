//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1172/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1172<F: Float>(t1882: F, t35047: F, t2179: F, t3565: F, t7407: F, t35084: F, t8392: F, t35162: F, t35185: F, t35033: F, t9276: F, t106300: F, t107627: F, t139675: F, t140364: F, t140370: F, t140376: F, t144: F, t1901: F, t2142: F, t27216: F, t27221: F, t27263: F, t33192: F, t3408: F, t3425: F, t3455: F, t35110: F, t3590: F, t446: F, t49622: F, t574: F, t5842: F, t5935: F, t605: F, t63755: F, t6725: F, t7339: F, t7357: F, t7414: F) -> (F, F, F) {
    let t149086 = t1882 * t35047;
    let t149093 = t2179 * t7407 * t3565;
    let t149101 = t8392 * t35084;
    let t149110 = t1882 * t35162;
    let t149112 = t1882 * t35185;
    let t149120 = t9276 * t35033;
    let t149129 = -t446 * t574 * t7414 * t3408 / F::new(3.0) + t446 * t574 * t2142 * t35110 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t1901 * t106300 * t27216 + F::new(4.0) / F::new(27.0) * t1901 * t107627 * t27221 + F::new(8.0) / F::new(3.0) * t1901 * t63755 * t7357 * t3455 + t149086 / F::new(27.0) + F::new(2.0) / F::new(3.0) * t446 * t574 * t5935 * t27263 + F::new(2.0) / F::new(3.0) * t446 * t144 * t149093 - F::new(2.0) / F::new(3.0) * t446 * t574 * t6725 * t5842 + F::new(2.0) / F::new(27.0) * t149101 + t1901 * t139675 * t3425 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t49622 * t33192 - t140364 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t149110 + F::new(2.0) / F::new(9.0) * t149112 - F::new(2.0) / F::new(9.0) * t140370 + t446 * t574 * t605 * t7339 * t3565 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t144 * t149120 + t140376 / F::new(9.0) - t446 * t574 * t3590 * t7339 / F::new(3.0);
    (t149093, t149120, t149129)
}
