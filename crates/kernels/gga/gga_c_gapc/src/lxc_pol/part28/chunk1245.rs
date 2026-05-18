//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1245/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1245<F: Float>(t20171: F, t33287: F, t5708: F, t19533: F, t19535: F, t11587: F, t11591: F, t3060: F, t28006: F, t3112: F, t33498: F, t8362: F) -> (F, F, F, F) {
    let t34764 = t5708 * t33287 * t20171;
    let t34767 = t19533 * t33287 * t19535;
    let t34772 = t3060 * t11587 * t11591;
    let t34776 = t3112 * t33498 * t8362 * t28006;
    (t34764, t34767, t34772, t34776)
}
