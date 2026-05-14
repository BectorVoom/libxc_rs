//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1198/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1198<F: Float>(t1882: F, t31213: F, t31246: F, t31139: F, t110293: F, t110294: F, t13885: F, t1449: F, t18139: F, t18412: F, t18617: F, t18641: F, t18730: F, t18741: F, t1901: F, t24789: F, t24793: F, t2574: F, t28255: F, t28267: F, t28355: F, t3837: F, t3977: F, t4005: F, t446: F, t6154: F, t6161: F, t6837: F, t729: F, t762: F, t97777: F) -> (F,) {
    let t122263 = t1882 * t31213;
    let t122265 = t1882 * t31246;
    let t122267 = t1882 * t31139;
    let t122273 = -2.0 / 3.0 * t446 * t729 * t4005 * t6837 - 2.0 / 9.0 * t1901 * t97777 * t18412 - t110293 + 4.0 / 27.0 * t110294 - 2.0 / 3.0 * t1901 * t13885 * t6161 * t18641 + 2.0 / 3.0 * t446 * t729 * t3977 * t28255 - 2.0 / 3.0 * t446 * t2574 * t6154 * t18617 - 4.0 / 3.0 * t1901 * t13885 * t28355 * t3837 + t1901 * t24793 * t18741 / 9.0 + t446 * t729 * t762 * t1449 * t18139 / 3.0 + t1901 * t24789 * t18730 / 9.0 - 2.0 / 9.0 * t122263 + 2.0 / 9.0 * t122265 + 2.0 / 27.0 * t122267 + 2.0 / 3.0 * t446 * t729 * t3977 * t28267;
    (t122273,)
}
