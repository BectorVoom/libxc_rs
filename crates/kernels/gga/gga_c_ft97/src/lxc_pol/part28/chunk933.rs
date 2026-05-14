//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 933/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk933<F: Float>(t136226: F, t136229: F, t144892: F, t144895: F, t144899: F, t144904: F, t144908: F, t144912: F, t144917: F, t144919: F, t144923: F, t144926: F, t144930: F, t144933: F, t144935: F, t144941: F) -> (F,) {
    let t145824 = -t144892 / 3.0 - 2.0 / 9.0 * t144895 - 2.0 * t144899 + 4.0 / 9.0 * t144904 - 2.0 * t144908 + t144912 + t144917 / 9.0 + 2.0 / 27.0 * t144919 - 4.0 / 9.0 * t144923 + t144926 / 18.0 - t136226 / 3.0 + t144930 / 18.0 - t144933 / 3.0 - t144935 / 9.0 + t136229 / 18.0 - t144941 / 6.0;
    (t145824,)
}
