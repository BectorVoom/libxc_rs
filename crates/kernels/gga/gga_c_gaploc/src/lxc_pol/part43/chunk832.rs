//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 832/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk832<F: Float>(t2615: F, t326: F, t47294: F, t13871: F, t1628: F, t813: F, t13875: F, t833: F, t13879: F, t317: F, t769: F, t40640: F, t40641: F, t43069: F, t43071: F, t43072: F, t43075: F, t43076: F, t43077: F, t43078: F, t43079: F, t43080: F) -> (F, F, F, F, F) {
    let t47299 = t2615 * t326 * t47294;
    let t47303 = 0.30674340763136599741e1 * t813 * t1628 * t13871;
    let t47306 = 0.30674340763136599741e1 * t833 * t1628 * t13875;
    let t47309 = 0.35750489951850426669e0 * t769 * t13879 * t317;
    let t47311 = t43069 - t43071 + t43072 / 2.0 + t40640 - t40641 + t43075 + t43076 - t43077 + t43078 - t43079 - t43080;
    (t47299, t47303, t47306, t47309, t47311)
}
