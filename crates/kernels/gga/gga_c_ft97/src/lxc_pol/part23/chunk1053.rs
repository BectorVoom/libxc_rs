//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1053/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1053<F: Float>(t1508: F, t5299: F, t840: F, t1248: F, t7124: F, t2843: F, t296: F, t1501: F, t5393: F, t5330: F, t6353: F, t1901: F, t25252: F, t29287: F, t29332: F, t29334: F, t29340: F, t29354: F, t29383: F, t31804: F, t31808: F, t31816: F, t31820: F, t31825: F, t31828: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31832 = t840 * t1508 * t5299;
    let t31835 = t7124 * t1248;
    let t31836 = t2843 * t31835;
    let t31837 = t296 * t31836;
    let t31841 = t1501 * t5393;
    let t31842 = t2843 * t31841;
    let t31843 = t296 * t31842;
    let t31847 = t840 * t6353 * t5330;
    let t31851 = 2.0 / 9.0 * t1901 * t31804 - 4.0 / 3.0 * t1901 * t31808 - 4.0 / 9.0 * t29287 - 4.0 / 9.0 * t29332 - 2.0 / 9.0 * t29334 - 2.0 / 3.0 * t446 * t31816 - 2.0 * t446 * t31820 + t25252 + 2.0 / 9.0 * t29340 - t446 * t31825 / 3.0 - 2.0 / 3.0 * t446 * t31828 - t446 * t31832 / 3.0 + 4.0 / 3.0 * t446 * t31837 + 2.0 / 27.0 * t29354 + 2.0 / 3.0 * t446 * t31843 + 2.0 / 3.0 * t446 * t31847 - 2.0 / 9.0 * t29383;
    (t31832, t31835, t31836, t31837, t31841, t31842, t31843, t31847, t31851)
}
