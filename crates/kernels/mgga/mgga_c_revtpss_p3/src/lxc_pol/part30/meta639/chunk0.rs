//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2216/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2216<F: Float>(t101218: F, t2122: F, t101204: F, t101234: F, t101237: F, t101240: F, t101243: F, t101252: F, t101360: F, t10309: F, t2121: F, t2123: F, t25162: F, t26792: F, t26795: F, t28093: F, t28147: F, t28154: F, t607: F, t7576: F, t7579: F, t96752: F, t96757: F, t96804: F) -> F {
    let t104332 = t2122 * t101218;
    let t104359 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t104332 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101237 * t26795 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101240 * t26795 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101243 * t26795 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t96757 - F::cast_from(5.0_f64) * t26792 * t101204 - t101360 * t2123 / F::cast_from(6.0_f64) - t28093 * t7576 / F::cast_from(3.0_f64) - t28093 * t7579 / F::cast_from(3.0_f64) + F::cast_from(35.0_f64) * t96804 * t101234 + F::cast_from(10.0_f64) * t101252 * t96752 + F::cast_from(20.0_f64) * t10309 * t607 * t2121 * t28147;
    t104359
}
