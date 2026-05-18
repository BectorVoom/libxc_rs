//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 921/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk921<F: Float>(t1248: F, t12831: F, t13607: F, t1249: F, t12925: F, t398: F, t963: F, t1163: F, t13522: F, t13526: F, t13530: F, t13533: F, t13536: F, t13540: F, t13543: F, t13546: F, t13549: F, t13552: F, t13555: F) -> (F, F, F, F, F) {
    let t13609 = t1248 * t13607 * t12831;
    let t13612 = t1248 * t1249 * t12925;
    let t13614 = t963 * t398;
    let t13616 = t1248 * t13614 * t1163;
    let t13618 = F::new(28.0) / F::new(27.0) * t13522;
    let t13629 = -t13618 - F::new(4.0) / F::new(9.0) * t13526 + F::new(2.0) / F::new(9.0) * t13530 - F::new(2.0) / F::new(3.0) * t13533 + t13536 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t13540 + F::new(4.0) / F::new(3.0) * t13543 - F::new(2.0) / F::new(3.0) * t13546 - F::new(2.0) * t13549 + F::new(2.0) * t13552 - t13555 / F::new(3.0);
    (t13609, t13612, t13614, t13616, t13629)
}
