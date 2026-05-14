//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1312/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1312<F: Float>(t119682: F, t119687: F, t119692: F, t119694: F, t119697: F, t119700: F, t119704: F, t119707: F, t119710: F, t119714: F, t119718: F, t119722: F, t119729: F, t119731: F, t119733: F, t119737: F, t119740: F, t119745: F, t119748: F, t119752: F, t119755: F, t119758: F, t119762: F) -> (F, F) {
    let t120928 = -t119682 / 18.0 + t119687 / 12.0 + t119692 / 3.0 + t119694 / 24.0 - t119697 / 36.0 + 2.0 / 3.0 * t119700 + t119704 / 18.0 + 8.0 / 9.0 * t119707 - 8.0 / 27.0 * t119710 + t119714 / 27.0 + t119718 / 18.0 + t119722 / 18.0;
    let t120942 = 2.0 / 9.0 * t119729 + t119731 / 9.0 - t119733 / 81.0 - 4.0 / 9.0 * t119737 + 4.0 / 27.0 * t119740 - t119745 / 8.0 + 2.0 / 3.0 * t119748 - 4.0 / 9.0 * t119752 - 8.0 / 9.0 * t119755 + 4.0 / 9.0 * t119758 + 10.0 / 81.0 * t119762;
    (t120928, t120942)
}
