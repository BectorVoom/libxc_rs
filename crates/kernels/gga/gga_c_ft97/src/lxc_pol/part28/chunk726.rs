//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 726/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk726<F: Float>(t28: F, t32917: F, t5890: F, t1369: F, t376: F, t7374: F, t2: F, t7312: F) -> (F, F, F, F) {
    let t32919 = t5890 * t28 * t32917;
    let t32922 = t1369 * t376 * t7374;
    let t32923 = t32922 / 3.0;
    let t32924 = t2 * t7312;
    (t32919, t32922, t32923, t32924)
}
