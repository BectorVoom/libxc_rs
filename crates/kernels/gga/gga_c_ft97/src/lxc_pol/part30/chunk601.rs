//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 601/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk601<F: Float>(t2404: F, t743: F, t24519: F, t3886: F, t6118: F, t684: F, t6852: F, t24432: F, t24438: F, t6878: F, t24437: F, t747: F, t992: F) -> (F, F, F, F, F, F, F) {
    let t27762 = t2404 * t743;
    let t27763 = t24519 * t3886;
    let t27764 = t27762 * t27763;
    let t27765 = t6118 * t27764;
    let t27767 = t6852 * t684;
    let t27768 = t24432 * t27767;
    let t27769 = t6118 * t27768;
    let t27772 = t24438 * t6878 * t684;
    let t27773 = t24437 * t27772;
    let t27775 = t992 * t747;
    (t27762, t27763, t27765, t27767, t27769, t27773, t27775)
}
