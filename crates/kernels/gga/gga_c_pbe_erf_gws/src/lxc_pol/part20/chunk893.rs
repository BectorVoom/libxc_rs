//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 893/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk893<F: Float>(t10272: F, t153: F, t156: F, t4550: F, t4554: F, t4557: F, t4568: F, t4600: F, t5574: F, t5582: F, t5585: F, t5588: F, t5592: F, t5595: F, t8064: F, t8066: F) -> (F,) {
    let t11178 = 0.13287210228946179141e1 * t5585 + t5592 - t5595 + t8064 - 0.1061188859155979109e0 * t8066 + 0.42708890021612718669e0 * t153 * t156 * t10272 - t4550 + 0.16752564107100880375e0 * t4554 + t4557 - 0.83762820535504401876e-1 * t4568 - t4600 - 0.53059442957798955448e-1 * t5574 - 0.16752564107100880375e0 * t5582 + t5588;
    (t11178,)
}
