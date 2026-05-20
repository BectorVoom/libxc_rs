//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3028/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3028<F: Float>(t4423: F, t836: F, t14741: F, t2710: F, t2713: F, t10744: F, t14861: F, t808: F, t40791: F, t4442: F, t14468: F, t236: F, t807: F, t854: F) -> (F, F, F, F, F) {
    let t51049 = t4423 * t836;
    let t51055 = t2710 * t2713 * t14741;
    let t51058 = t10744 * t808 * t14861;
    let t51060 = t40791 * t4442;
    let t51070 = t807 * t236 * t854 * t14468;
    (t51049, t51055, t51058, t51060, t51070)
}
