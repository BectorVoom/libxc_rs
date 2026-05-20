//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1729/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1729<F: Float>(t16712: F, t12256: F, t1469: F, t3362: F, t4186: F, t3367: F, t3153: F, t5284: F, t300: F, t5155: F, t16710: F, t16708: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16713 = F::cast_from(0.9877777777777777778e-2_f64) * t16712;
    let t16714 = t12256 * t1469;
    let t16724 = t3362 * t4186;
    let t16737 = t3367 * t4186;
    let t16756 = t5284 * t3153;
    let t16784 = t300 * t5155;
    let t16797 = F::cast_from(0.23744444444444444444e-1_f64) * t16710;
    let t16798 = F::cast_from(0.11872222222222222222e-1_f64) * t16712;
    let t16820 = F::cast_from(0.41203703703703703704e-2_f64) * t16708;
    (t16713, t16714, t16724, t16737, t16756, t16784, t16797, t16798, t16820)
}
