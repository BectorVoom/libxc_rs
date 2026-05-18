//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 922/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk922<F: Float>(t209: F, t42577: F, t42621: F, t42680: F, t42723: F, t42776: F, t42818: F, t42861: F, t42900: F, t29650: F, t2972: F, t13235: F, t14537: F) -> (F, F, F) {
    let t42904 = (t42577 + t42621 + t42680 + t42723 + t42776 + t42818 + t42861 + t42900) * t209;
    let t42906 = F::new(2.0) * t29650 * t2972;
    let t42908 = F::new(6.0) * t14537 * t13235;
    (t42904, t42906, t42908)
}
