//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 840/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk840<F: Float>(t1307: F, t1588: F, t8270: F, t1317: F, t28: F, t22956: F, t22961: F, t22965: F, t22968: F, t22973: F, t22978: F, t22980: F, t22984: F, t22989: F, t22991: F, t22996: F, t23001: F, t23006: F, t23013: F, t23016: F) -> (F, F, F, F) {
    let t23018 = t1307 * t1588;
    let t23019 = t8270 * t23018;
    let t23021 = t1317 * t28 * t23019;
    let t23023 = -t22956 / 6.0 - 2.0 / 3.0 * t22961 - 6.0 * t22965 - 4.0 / 3.0 * t22968 + 2.0 * t22973 + t22978 - 2.0 / 3.0 * t22980 + t22984 / 3.0 + 2.0 / 9.0 * t22989 - 2.0 / 9.0 * t22991 + 2.0 / 3.0 * t22996 + t23001 / 2.0 + t23006 / 4.0 - 3.0 / 8.0 * t23013 - t23016 / 6.0 - 3.0 * t23021;
    (t23018, t23019, t23021, t23023)
}
