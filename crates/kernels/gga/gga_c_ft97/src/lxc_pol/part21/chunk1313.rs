//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1313/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1313<F: Float>(t105467: F, t105989: F, t105990: F, t105997: F, t119766: F, t119770: F, t119773: F, t119776: F, t119780: F, t119783: F, t119788: F, t119792: F, t106009: F, t106011: F, t119796: F, t119799: F, t119803: F, t119807: F, t119810: F, t119814: F, t119817: F, t119819: F, t119823: F, t95100: F) -> (F, F) {
    let t120951 = 8.0 / 27.0 * t119766 - 4.0 / 9.0 * t119770 - 4.0 / 9.0 * t119773 + 4.0 / 3.0 * t119776 - 4.0 / 9.0 * t119780 - 8.0 / 9.0 * t119783 - t119788 + t105989 + t105990 + 8.0 / 81.0 * t105467 - t105997 + 5.0 / 81.0 * t119792;
    let t120962 = 2.0 / 81.0 * t95100 - t119796 / 27.0 + t119799 / 18.0 - t119803 - t119807 / 3.0 + t119810 / 9.0 - t106009 + t106011 - 2.0 * t119814 - 2.0 / 9.0 * t119817 - t119819 / 27.0 - t119823 / 3.0;
    (t120951, t120962)
}
