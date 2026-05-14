//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 528/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk528<F: Float>(t1349: F, t1362: F, t1389: F, t149: F, t564: F, t5766: F, t5771: F, t5772: F, t5775: F, t5781: F, t5845: F, t5849: F, t5936: F, t5952: F, t5957: F, t5969: F, t5973: F, t5975: F) -> (F,) {
    let t5981 = t5766 * t1362 / 6.0 - t5771 - t5772 * t5775 / 18.0 - t1349 * t5781 / 3.0 + t1349 * t5845 / 6.0 + t1349 * t5849 / 6.0 - t564 * t1389 - t149 * t5973 + 2.0 * t5975 - 2.0 * t5936 - 2.0 * t5952 + 4.0 * t5957 - 2.0 * t5969;
    (t5981,)
}
