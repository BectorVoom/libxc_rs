//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1222/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1222<F: Float>(t101767: F, t101744: F, t101751: F, t101754: F, t101758: F, t101761: F, t101764: F, t93522: F, t93524: F, t93530: F, t93542: F, t101771: F, t101778: F, t101781: F, t101775: F, t101787: F, t101791: F, t101795: F, t101799: F, t101803: F, t101807: F, t93558: F, t93561: F) -> (F, F) {
    let t102226 = t101767 / 6.0;
    let t102227 = t101744 / 3.0 + t93522 + t93524 + t93530 - t93542 + 2.0 / 3.0 * t101751 - 2.0 / 3.0 * t101754 - t101758 / 6.0 - 8.0 / 3.0 * t101761 + 8.0 / 9.0 * t101764 - t102226;
    let t102228 = 4.0 / 3.0 * t101771;
    let t102230 = 4.0 / 3.0 * t101778;
    let t102231 = 4.0 / 3.0 * t101781;
    let t102237 = -t102228 + 2.0 * t101775 - t102230 - t102231 - t93558 - t93561 - 12.0 * t101787 + 4.0 / 3.0 * t101791 - 3.0 / 8.0 * t101795 - 6.0 * t101799 - t101803 - t101807 / 6.0;
    (t102227, t102237)
}
