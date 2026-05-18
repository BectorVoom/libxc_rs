//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 548/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk548<F: Float>(t1301: F, t22855: F, t1300: F, t1309: F, t1637: F, t1586: F, t5617: F) -> (F, F, F, F) {
    let t22856 = t1301 * t22855;
    let t22858 = F::new(0.42562405586419753087e-2) * t1300 * t22856;
    let t22870 = t1637 * t1309;
    let t22873 = t1586 * t5617;
    (t22856, t22858, t22870, t22873)
}
