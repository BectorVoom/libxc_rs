//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1351/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1351<F: Float>(t113458: F, t113459: F, t126946: F, t28729: F, t2680: F, t31551: F, t193: F, t824: F, t89: F, t10570: F, t126487: F, t1486: F, t113513: F, t1234: F, t2347: F, t3886: F, t99528: F) -> (F, F, F, F, F) {
    let t126949 = t113458 * t113459 * t126946 * t28729;
    let t126951 = t2680 * t31551;
    let t126954 = t89 * t193 * t126951 * t824;
    let t126958 = t1486 * t193 * t10570 * t126487;
    let t126963 = t99528 * t113513 * t1234 * t2347 * t3886;
    (t126949, t126951, t126954, t126958, t126963)
}
