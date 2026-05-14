//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1387/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1387<F: Float>(t127836: F, t127839: F, t127842: F, t127846: F, t127849: F, t127852: F, t127855: F, t127858: F, t127861: F, t127866: F, t127869: F, t127872: F, t126118: F, t1486: F, t193: F, t2781: F) -> (F, F) {
    let t127874 = -2.0 / 3.0 * t127836 + 5.0 / 27.0 * t127839 - 4.0 / 9.0 * t127842 + 3.0 / 2.0 * t127846 + t127849 - t127852 / 6.0 - t127855 / 3.0 - t127858 / 3.0 + t127861 / 9.0 - t127866 / 18.0 - 4.0 / 3.0 * t127869 + 4.0 / 9.0 * t127872;
    let t127877 = t1486 * t193 * t2781 * t126118;
    (t127874, t127877)
}
