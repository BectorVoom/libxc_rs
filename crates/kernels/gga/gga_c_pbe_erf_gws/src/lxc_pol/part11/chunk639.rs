//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 639/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk639<F: Float>(t1062: F, t1903: F, t2519: F, t713: F, t1009: F, t4991: F, t587: F, t1022: F, t1697: F, t197: F, t5283: F, t1802: F, t1885: F, t1061: F, t1923: F, t256: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7541 = t1062 * t1903;
    let t7573 = t2519 * t713;
    let t7579 = t4991 * t1009;
    let t7580 = t587 * t7579;
    let t7651 = t1022 * t1697;
    let t7669 = t5283 * t197;
    let t7703 = t1885 * t1802;
    let t7733 = t1061 * t1923;
    let t7734 = t7733 * t256;
    (t7541, t7573, t7579, t7580, t7651, t7669, t7703, t7733, t7734)
}
