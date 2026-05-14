//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1274/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1274<F: Float>(t119785: F, t1369: F, t28: F, t9236: F, t105340: F, t119760: F, t5899: F, t105458: F, t105460: F, t105467: F, t105483: F, t119766: F, t119770: F, t119773: F, t119776: F, t119780: F, t119783: F) -> (F, F, F) {
    let t119788 = t1369 * t28 * t9236 * t119785;
    let t119792 = t5899 * t105340 * t119760;
    let t119794 = 8.0 / 9.0 * t119766 - 4.0 / 3.0 * t119770 - 4.0 / 3.0 * t119773 + 4.0 * t119776 - 4.0 / 3.0 * t119780 - 8.0 / 3.0 * t119783 - 3.0 * t119788 + t105458 + t105460 + 8.0 / 27.0 * t105467 - t105483 + 5.0 / 27.0 * t119792;
    (t119788, t119792, t119794)
}
