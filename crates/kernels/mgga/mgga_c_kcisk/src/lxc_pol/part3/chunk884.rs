//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 884/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk884<F: Float>(t1175: F, t12970: F, t12992: F, t13244: F, t13247: F, t13250: F, t13253: F, t13274: F, t1355: F, t306: F, t3559: F, t3587: F, t3599: F, t3602: F) -> F {
    let t13277 = F::new(3.0) / F::new(16.0) * t13244 * t12970 - F::new(3.0) / F::new(8.0) * t13247 * t3559 - F::new(3.0) / F::new(8.0) * t3599 * t13250 + F::new(3.0) / F::new(4.0) * t13253 * t1175 + F::new(3.0) / F::new(4.0) * t3602 * t3587 + t1355 * t12992 / F::new(4.0) + t306 * t13274 / F::new(2.0);
    t13277
}
