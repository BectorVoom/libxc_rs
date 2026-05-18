//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1024/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1024<F: Float>(t1871: F, t22952: F, t26016: F, t5675: F, t34384: F, t379: F, t22958: F, t5674: F, t136159: F, t136188: F, t25883: F, t32069: F) -> (F, F, F, F) {
    let t144892 = t22952 * t1871 * t5675 * t26016;
    let t144893 = t34384 * t379;
    let t144895 = t5674 * t22958 * t144893;
    let t144899 = t136159 * t136188 * t32069 * t25883;
    (t144892, t144893, t144895, t144899)
}
