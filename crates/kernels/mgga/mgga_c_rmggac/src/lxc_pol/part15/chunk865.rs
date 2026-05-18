//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 865/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk865<F: Float>(t38279: F, t38280: F, t8653: F, t8657: F, t8660: F, t9381: F, t9934: F, t9937: F, t9940: F, t9946: F, t9950: F, t38292: F, t8673: F, t8681: F, t8683: F, t9412: F, t9977: F, t9979: F, t9981: F, t9983: F, t9987: F, t9992: F) -> (F, F) {
    let t44554 = -t38279 - t38280 - F::new(0.25538759935978703639e-4) * t8653 - t9381 + t9934 + t9937 + t9940 + t9946 - F::new(0.36366215538993788972e-1) * t8657 + F::new(0.20455996240684006297e-1) * t8660 + t9950;
    let t44563 = t9977 - t9979 - t9981 - t9983 - t9987 + t9992 + F::new(0.14546486215597515588e0) * t8673 - t9412 - t38292 + F::new(0.25538759935978703639e-4) * t8681 - F::new(0.25538759935978703639e-4) * t8683;
    (t44554, t44563)
}
