//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1434/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1434<F: Float>(t33893: F, t33920: F, t36749: F, t36750: F, t36751: F, t36752: F, t36753: F, t36754: F, t36755: F, t36756: F, t36758: F, t33935: F, t36761: F, t36762: F, t36763: F, t36765: F, t36766: F, t36767: F, t36768: F, t36769: F, t36770: F, t36771: F) -> (F, F) {
    let t38774 = F::cast_from(0.24598298249421296296e-6_f64) * t33893 - t36749 - t36750 + t36751 - t36752 + t36753 - t36754 + t36755 - t36756 + F::cast_from(0.50595483470764842602e-7_f64) * t33920 + t36758;
    let t38777 = t36761 - t36762 + t36763 + F::cast_from(0.25301106770833333334e-5_f64) * t33935 + t36765 + t36766 + t36767 + t36768 + t36769 - t36770 - t36771;
    (t38774, t38777)
}
