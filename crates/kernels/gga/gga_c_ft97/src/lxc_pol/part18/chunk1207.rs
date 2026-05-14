//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1207/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1207<F: Float>(t22873: F, t28: F, t3103: F, t89: F, t6454: F, t7241: F, t1588: F, t1651: F, t22952: F, t22953: F, t6495: F, t1643: F, t25928: F, t22993: F, t25985: F, t101812: F, t101814: F, t101817: F, t101820: F, t101824: F, t101827: F, t101831: F) -> (F, F, F, F, F, F, F) {
    let t101835 = t89 * t28 * t22873 * t3103;
    let t101837 = t7241 * t6454;
    let t101840 = t89 * t28 * t101837 * t1588;
    let t101844 = t22952 * t22953 * t6495 * t1651;
    let t101848 = t22952 * t25928 * t6495 * t1643;
    let t101852 = t22952 * t22953 * t22993 * t25985;
    let t101854 = t101812 - 4.0 / 9.0 * t101814 + 4.0 / 27.0 * t101817 - 4.0 / 9.0 * t101820 - t101824 - 4.0 * t101827 + 2.0 / 3.0 * t101831 + 4.0 / 3.0 * t101835 - 2.0 * t101840 - t101844 / 36.0 - t101848 / 54.0 - t101852 / 18.0;
    (t101835, t101837, t101840, t101844, t101848, t101852, t101854)
}
