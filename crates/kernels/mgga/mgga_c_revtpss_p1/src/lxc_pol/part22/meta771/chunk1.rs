//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2857/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2857<F: Float>(t12904: F, t3708: F, t11262: F, t1247: F, t3590: F, t3610: F, t3612: F, t1231: F, t12898: F, t3651: F, t3655: F, t43813: F) -> (F, F, F, F, F, F) {
    let t44270 = t3708 * t12904;
    let t44273 = t1247 * t11262 * t3590;
    let t44276 = t3610 * t11262 * t3612;
    let t44291 = t1231 * t12898;
    let t44293 = t3651 * t3655;
    let t44307 = F::cast_from(0.86419753086419753087e-1_f64) * t43813;
    (t44270, t44273, t44276, t44291, t44293, t44307)
}
