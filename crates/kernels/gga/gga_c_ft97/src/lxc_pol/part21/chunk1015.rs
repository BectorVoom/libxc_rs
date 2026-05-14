//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1015/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1015<F: Float>(t1526: F, t4656: F, t7705: F, t11280: F, t1527: F, t15567: F, t15625: F, t15737: F, t15742: F, t15746: F, t16633: F, t16682: F, t16694: F, t16708: F, t16712: F, t16719: F, t16732: F, t1943: F, t3088: F, t61123: F, t64621: F, t64623: F, t64631: F, t64642: F, t64655: F, t78650: F, t78653: F, t78678: F) -> (F,) {
    let t78681 = t1526 * t7705 * t4656;
    let t78693 = t78650 / 18.0 - t78653 / 36.0 - t1526 * t1527 * t16682 / 12.0 + t1526 * t1527 * t16712 / 6.0 - t1526 * t1527 * t16694 / 12.0 - t1526 * t1527 * t1943 * t15625 / 12.0 + t64621 - t64623 - t1526 * t11280 * t16708 / 3.0 - t1526 * t3088 * t16719 / 9.0 - t1526 * t1527 * t16732 / 6.0 - t78678 / 27.0 - t78681 / 18.0 + 2.0 / 3.0 * t15567 * t16633 * t15742 - 7.0 / 27.0 * t15567 * t64631 * t15737 - 4.0 / 9.0 * t61123 * t16633 * t15746 - t64642 / 9.0 - t64655;
    (t78693,)
}
