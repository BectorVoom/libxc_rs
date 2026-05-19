//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 967/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk967<F: Float>(t10814: F, t1022: F, t7284: F, t787: F, t2639: F, t10627: F, t723: F) -> (F, F, F, F, F) {
    let t10815 = F::cast_from(0.29792074959875355558e-1_f64) * t10814;
    let t10816 = t7284 * t1022;
    let t10817 = t787 * t10816;
    let t10819 = F::cast_from(0.25025342966295298669e1_f64) * t10817 * t2639;
    let t10820 = t10627 * t723;
    (t10815, t10816, t10817, t10819, t10820)
}
