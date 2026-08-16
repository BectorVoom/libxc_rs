//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 674/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk674<F: Float>(t24981: F, t28520: F, t6317: F, t2404: F, t852: F, t28524: F, t1212: F, t856: F, t2862: F, t6318: F, t24980: F, t10683: F, t4162: F) -> (F, F, F, F, F, F) {
    let t28769 = t24981 * t28520;
    let t28770 = t6317 * t28769;
    let t28772 = t2404 * t852;
    let t28773 = t28772 * t28524;
    let t28774 = t6317 * t28773;
    let t28776 = t1212 * t856;
    let t28778 = t2862 * t6318 * t28776;
    let t28779 = t24980 * t28778;
    let t28782 = t10683 * t6318 * t4162;
    (t28770, t28772, t28774, t28776, t28779, t28782)
}
