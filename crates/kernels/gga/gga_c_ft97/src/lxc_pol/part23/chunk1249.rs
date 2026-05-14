//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1249/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1249<F: Float>(t124003: F, t41816: F, t446: F, t18497: F, t24519: F, t3281: F, t9744: F, t2354: F, t24546: F, t4973: F, t6118: F, t109414: F, t17790: F, t96935: F, t123980: F, t123983: F, t123986: F, t123989: F, t123992: F, t123995: F, t123998: F, t124001: F) -> (F, F, F, F, F, F) {
    let t124005 = t446 * t41816 * t124003;
    let t124007 = t24519 * t18497;
    let t124009 = t3281 * t9744 * t124007;
    let t124013 = t6118 * t2354 * t24546 * t4973;
    let t124016 = t109414 * t96935 * t17790;
    let t124018 = -4.0 / 9.0 * t123980 + 8.0 / 9.0 * t123983 - 4.0 / 9.0 * t123986 - 2.0 / 9.0 * t123989 - 2.0 / 9.0 * t123992 + 2.0 / 27.0 * t123995 + 2.0 / 3.0 * t123998 - 4.0 / 27.0 * t124001 + 10.0 / 81.0 * t124005 - 8.0 / 27.0 * t124009 + t124013 / 18.0 + 4.0 / 9.0 * t124016;
    (t124005, t124007, t124009, t124013, t124016, t124018)
}
