//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1202/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1202<F: Float>(t27491: F, t6048: F, t12345: F, t1555: F, t28573: F, t17320: F, t94833: F, t48044: F, t7943: F, t28644: F, t4189: F, t51125: F, t585: F) -> (F, F, F, F, F, F) {
    let t97647 = F::cast_from(2.0_f64) * t27491 * t6048;
    let t97650 = F::cast_from(12.0_f64) * t12345 * t28573 * t1555;
    let t97652 = F::cast_from(6.0_f64) * t94833 * t17320;
    let t97654 = F::cast_from(4.0_f64) * t48044 * t7943;
    let t97657 = F::cast_from(4.0_f64) * t4189 * t28644 * t1555;
    let t97661 = t51125 * t585;
    (t97647, t97650, t97652, t97654, t97657, t97661)
}
