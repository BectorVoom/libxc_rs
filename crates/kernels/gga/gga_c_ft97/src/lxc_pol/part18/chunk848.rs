//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 848/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk848<F: Float>(t23084: F, t488: F, t376: F, t5623: F, t1286: F, t1332: F, t1853: F, t8418: F, t22956: F, t22961: F, t22965: F, t22968: F, t22973: F, t22978: F, t22980: F, t22984: F, t22989: F, t22991: F, t22996: F, t23001: F, t23006: F, t23013: F, t23016: F, t23021: F) -> (F, F, F, F, F, F) {
    let t23085 = t488 * t23084;
    let t23089 = t376 * t5623;
    let t23090 = t1286 * t23089;
    let t23092 = t1332 * t1853;
    let t23093 = t8418 * t23092;
    let t23110 = -t22956 / 18.0 - 2.0 / 9.0 * t22961 - 2.0 * t22965 - 4.0 / 9.0 * t22968 + 2.0 / 3.0 * t22973 + t22978 / 3.0 - 2.0 / 9.0 * t22980 + t22984 / 9.0 + 2.0 / 27.0 * t22989 - 2.0 / 27.0 * t22991 + 2.0 / 9.0 * t22996 + t23001 / 6.0 + t23006 / 12.0 - t23013 / 8.0 - t23016 / 18.0 - t23021;
    (t23085, t23089, t23090, t23092, t23093, t23110)
}
