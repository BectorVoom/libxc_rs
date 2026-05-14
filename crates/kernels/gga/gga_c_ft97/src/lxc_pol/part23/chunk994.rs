//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 994/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk994<F: Float>(t13520: F, t232: F, t24265: F, t24276: F, t24289: F, t24324: F, t27546: F, t27570: F, t27576: F, t27579: F, t27582: F, t27609: F, t27616: F, t27651: F, t27658: F, t30590: F, t30595: F, t30600: F, t30603: F, t30608: F, t30613: F, t30615: F, t30617: F, t30622: F, t30625: F, t30632: F, t30636: F, t30641: F, t30642: F, t6034: F, t6035: F, t6045: F) -> (F,) {
    let t30645 = -0.11491849508333333333e0 * t24324 * t6045 * t30590 + 0.14846767889314528222e-3 * t24276 * t30595 + 0.10560293360415908094e-4 * t27616 * t30600 + 0.89080607335887169332e-3 * t27609 * t232 * t30603 - 0.89080607335887169332e-3 * t24265 * t30608 - 0.42562405586419753086e-2 * t27570 + 2.0 * t30613 + 4.0 * t30615 - 0.25537443351851851852e-1 * t27651 * t6035 * t30617 - 0.30274029503828221194e-3 * t27658 * t30622 + 0.22270151833971792333e-3 * t6034 * t6035 * t30625 + t24289 - 0.30274029503828221194e-3 * t27576 + 0.25537443351851851852e-1 * t27579 - 0.25537443351851851852e-1 * t27582 - 0.51690243689028715488e-4 * t13520 * t30632 + 0.15322466011111111111e0 * t27546 * t6045 * t30636 - 0.27568129967481981592e-3 * t30641 * t30642;
    (t30645,)
}
