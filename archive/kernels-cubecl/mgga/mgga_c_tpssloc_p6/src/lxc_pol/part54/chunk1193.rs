//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1193/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1193<F: Float>(t2039: F, t24932: F, t27888: F, t31237: F, t31239: F, t31704: F, t31706: F, t31708: F, t31716: F, t31719: F, t31721: F, t32349: F, t32350: F, t671: F, t7056: F, t7266: F, t8446: F) -> F {
    let t32359 = F::cast_from(2.0_f64) * t2039 * t24932 + F::cast_from(2.0_f64) * t2039 * t27888 + F::cast_from(2.0_f64) * t32350 * t671 + F::cast_from(2.0_f64) * t7056 * t7266 + t31237 + t31239 + t31704 + t31706 + t31708 + t31716 + t31719 + t31721 + t32349 + t8446;
    t32359
}
