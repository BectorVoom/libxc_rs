//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 934/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk934<F: Float>(t713: F, t9587: F, t2354: F, t446: F, t41482: F, t724: F, t2594: F, t41464: F, t41823: F, t41829: F, t41831: F, t41835: F, t41839: F, t41844: F, t41846: F, t41852: F, t41856: F, t41859: F, t41863: F) -> (F, F, F, F, F) {
    let t41865 = t9587 * t713;
    let t41867 = t446 * t2354 * t41865;
    let t41870 = t446 * t724 * t41482;
    let t41873 = t446 * t2594 * t41464;
    let t41875 = -10.0 / 27.0 * t41823 + 4.0 / 3.0 * t41829 + 4.0 / 9.0 * t41831 - 2.0 / 3.0 * t41835 - 2.0 / 3.0 * t41839 + t41844 + 4.0 / 3.0 * t41846 + 4.0 * t41852 + t41856 + t41859 / 3.0 + 2.0 / 9.0 * t41863 + 4.0 / 3.0 * t41867 - 2.0 * t41870 + 4.0 / 3.0 * t41873;
    (t41865, t41867, t41870, t41873, t41875)
}
