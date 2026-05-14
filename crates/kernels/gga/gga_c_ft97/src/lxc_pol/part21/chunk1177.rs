//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1177/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1177<F: Float>(t1564: F, t25878: F, t25899: F, t3052: F, t102239: F, t102243: F, t116735: F, t116739: F, t116743: F, t116747: F, t116752: F, t116756: F, t116760: F, t116764: F, t116767: F, t23057: F, t4462: F, t5674: F) -> (F, F, F) {
    let t116771 = t25878 * t1564 * t25899 * t3052;
    let t116773 = 24.0 * t116735 - 12.0 * t116739 + 4.0 / 3.0 * t116743 - t116747 / 2.0 + 2.0 * t116752 - 6.0 * t116756 - t116760 - t116764 + t116767 / 4.0 + t102239 - t102243 + 2.0 / 3.0 * t116771;
    let t116776 = t5674 * t1564 * t23057 * t4462;
    (t116771, t116773, t116776)
}
