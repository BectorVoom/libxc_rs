//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1183/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1183<F: Float>(t47855: F, t47862: F, t47864: F, t47866: F, t47868: F, t47870: F, t47872: F, t47874: F, t47878: F, t47882: F, t47886: F, t47888: F, t47890: F, t47892: F, t47893: F, t47895: F, t47896: F, t47898: F, t47899: F, t47902: F, t47904: F, t47906: F) -> (F, F) {
    let t48656 = -t47855 + t47862 - t47864 + t47866 - t47868 + t47870 + t47872 - t47874 + t47878 + t47882 + t47886;
    let t48657 = -t47888 - t47890 - t47892 - t47893 - t47895 + t47896 - t47898 - t47899 - t47902 - t47904 - t47906;
    (t48656, t48657)
}
