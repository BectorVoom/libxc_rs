//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1270/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1270<F: Float>(t1969: F, t23652: F, t4462: F, t5899: F, t15772: F, t5900: F, t119682: F, t119687: F, t119692: F, t119695: F, t119698: F, t119700: F, t119704: F, t119707: F, t119710: F, t119714: F) -> (F, F, F) {
    let t119718 = t5899 * t1969 * t23652 * t4462;
    let t119722 = t5899 * t1969 * t5900 * t15772;
    let t119724 = -t119682 / 6.0 + t119687 / 4.0 + t119692 + t119695 - t119698 + 2.0 * t119700 + t119704 / 6.0 + 8.0 / 3.0 * t119707 - 8.0 / 9.0 * t119710 + t119714 / 9.0 + t119718 / 6.0 + t119722 / 6.0;
    (t119718, t119722, t119724)
}
