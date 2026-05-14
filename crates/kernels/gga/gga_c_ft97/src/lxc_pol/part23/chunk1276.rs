//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1276/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1276<F: Float>(t123980: F, t123983: F, t123986: F, t123989: F, t123992: F, t123995: F, t123998: F, t124001: F, t124005: F, t124009: F, t124013: F, t124016: F, t108250: F, t108261: F, t108263: F, t124020: F, t124026: F, t124031: F, t124036: F, t124040: F, t124045: F, t97328: F, t97330: F, t97338: F) -> (F, F) {
    let t124561 = -4.0 / 3.0 * t123980 + 8.0 / 3.0 * t123983 - 4.0 / 3.0 * t123986 - 2.0 / 3.0 * t123989 - 2.0 / 3.0 * t123992 + 2.0 / 9.0 * t123995 + 2.0 * t123998 - 4.0 / 9.0 * t124001 + 10.0 / 27.0 * t124005 - 8.0 / 9.0 * t124009 + t124013 / 6.0 + 4.0 / 3.0 * t124016;
    let t124568 = -4.0 / 9.0 * t124020 + t97328 - t97330 - t124026 / 6.0 + t97338 + 2.0 / 3.0 * t124031 - t108250 + t108261 + t108263 - t124036 / 2.0 + t124040 / 3.0 + t124045 / 3.0;
    (t124561, t124568)
}
