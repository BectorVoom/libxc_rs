//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1392/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1392<F: Float>(t101826: F, t101828: F, t101830: F, t101832: F, t101833: F, t101835: F, t101837: F, t101839: F, t101840: F, t101841: F, t102804: F, t102813: F, t102816: F, t102820: F, t102828: F, t102833: F, t102836: F, t102839: F, t102840: F, t103794: F, t103930: F, t187: F) -> F {
    let t103934 = t101826 - t101828 - t101830 - t101832 - t101833 + t101835 - t101837 - t101839 - t101840 - t101841 + t187 * (t102804 + t102840 + t103794 + t103930) + t102813 + t102816 + t102820 + t102828 + t102833 - t102836 - t102839;
    t103934
}
