//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1252/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1252<F: Float>(t165: F, t30105: F, t1349: F, t30118: F, t376: F, t104547: F, t6584: F, t2179: F, t3565: F, t6718: F, t12664: F, t26520: F, t1984: F, t104432: F, t104434: F, t104436: F, t104450: F, t1969: F, t26581: F, t26785: F, t26817: F, t28: F, t30130: F, t30162: F, t379: F, t5766: F, t5772: F, t5779: F, t609: F, t6589: F, t9439: F) -> (F, F, F, F) {
    let t119283 = t30105 * t165;
    let t119289 = t1349 * t376 * t30118;
    let t119294 = t104547 * t6584;
    let t119297 = t2179 * t6718 * t3565;
    let t119299 = t12664 * t26520;
    let t119308 = t1984 * t30105;
    let t119313 = -t5772 * t1969 * t119283 * t379 / 18.0 - t104432 - t104434 + 2.0 / 9.0 * t119289 + 4.0 / 27.0 * t104436 - t26817 * t26785 / 9.0 + t119294 / 27.0 + 8.0 * t119297 + 8.0 * t119299 + t5766 * t30162 / 6.0 + t104450 - 2.0 / 3.0 * t26581 * t6589 - 24.0 * t9439 * t30130 * t609 - t1349 * t28 * t119308 * t5779 / 3.0;
    (t119297, t119299, t119308, t119313)
}
