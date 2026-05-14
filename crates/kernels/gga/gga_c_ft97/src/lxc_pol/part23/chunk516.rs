//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 516/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk516<F: Float>(t6014: F, t6758: F, t1113: F, t203: F, t1411: F, t1127: F, t6018: F, t1109: F, t227: F, t52: F, t11: F, t1103: F, t41: F) -> (F, F, F, F, F, F) {
    let t6759 = t6014 * t6758;
    let t6762 = t203 * t1113;
    let t6763 = t6762 * t1411;
    let t6767 = t6018 * t1127;
    let t6774 = t52 * t227 * t1109;
    let t6776 = -0.1201569457037037037e0 * t41 * t11 * t1103 - 0.59273806478425129877e-2 * t6774;
    (t6759, t6762, t6763, t6767, t6774, t6776)
}
