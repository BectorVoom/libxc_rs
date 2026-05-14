//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 961/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk961<F: Float>(t28924: F, t871: F, t1466: F, t1506: F, t28843: F, t28845: F, t28848: F, t28850: F, t28852: F, t28855: F, t28857: F, t28860: F, t28863: F, t28870: F, t28874: F, t4027: F, t4135: F) -> (F, F) {
    let t28925 = t871 * t28924;
    let t28927 = 2.0 * t28843 + 4.0 * t28845 + 4.0 * t28848 + 4.0 * t28850 + 4.0 * t28852 + 4.0 * t28855 - 2.0 * t28857 - 2.0 * t28860 + t1466 * t28863 / 6.0 - t4135 * t1506 - t4027 * t1506 - t1466 * t28870 / 3.0 - t28874 / 18.0 - 2.0 * t28925;
    (t28925, t28927)
}
