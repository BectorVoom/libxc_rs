//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 931/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk931<F: Float>(t28524: F, t28772: F, t6317: F, t1212: F, t856: F, t2862: F, t6318: F, t24980: F, t10683: F, t4162: F, t25162: F, t7068: F, t18: F, t6334: F, t2665: F, t3281: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28773 = t28772 * t28524;
    let t28774 = t6317 * t28773;
    let t28776 = t1212 * t856;
    let t28778 = t2862 * t6318 * t28776;
    let t28779 = t24980 * t28778;
    let t28782 = t10683 * t6318 * t4162;
    let t28783 = t6317 * t28782;
    let t28784 = t25162 * t7068;
    let t28788 = t6334 * t18;
    let t28789 = t2665 * t28788;
    let t28790 = t3281 * t28789;
    (t28773, t28774, t28776, t28778, t28779, t28782, t28783, t28784, t28789, t28790)
}
