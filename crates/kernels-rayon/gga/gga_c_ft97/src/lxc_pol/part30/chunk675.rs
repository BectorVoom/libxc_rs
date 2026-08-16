//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 675/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk675(t28782: f64, t6317: f64, t25162: f64, t7068: f64, t18: f64, t6334: f64, t2665: f64, t3281: f64, t10683: f64, t7036: f64, t824: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t28783 = t6317 * t28782;
    let t28784 = t25162 * t7068;
    let t28788 = t6334 * t18;
    let t28789 = t2665 * t28788;
    let t28790 = t3281 * t28789;
    let t28793 = t10683 * t7036 * t824;
    let t28794 = t446 * t28793;
    (t28783, t28784, t28788, t28790, t28794)
}
