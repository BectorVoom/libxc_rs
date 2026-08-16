//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1518/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1518<F: Float>(t41864: F, t41867: F, t41871: F, t41873: F, t41876: F, t41879: F, t41882: F, t41885: F, t41888: F, t41942: F, t41947: F, t41949: F) -> F {
    let t42850 = -t41942 + t41947 + t41949 - t41864 - t41867 + t41871 + t41873 - t41876 - t41879 - t41882 - t41885 + t41888;
    t42850
}
