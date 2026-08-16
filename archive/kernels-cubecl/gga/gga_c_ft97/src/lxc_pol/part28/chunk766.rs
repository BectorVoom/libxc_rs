//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 766/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk766<F: Float>(t1852: F, t32411: F, t492: F, t7281: F, t22943: F, t5731: F, t7274: F, t8418: F, t1307: F, t1337: F, t1564: F, t379: F) -> (F, F, F, F, F, F, F, F) {
    let t32412 = t1852 * t32411;
    let t32414 = t7281 * t492;
    let t32415 = t1852 * t32414;
    let t32417 = t22943 * t5731;
    let t32419 = t7274 * t492;
    let t32420 = t8418 * t32419;
    let t32423 = t1307 * t1337;
    let t32425 = t1564 * t32423 * t379;
    (t32412, t32414, t32415, t32417, t32419, t32420, t32423, t32425)
}
