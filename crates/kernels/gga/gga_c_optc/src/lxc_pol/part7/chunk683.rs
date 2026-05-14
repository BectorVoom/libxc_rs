//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 683/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk683<F: Float>(t43: F, t2229: F, t758: F, t1972: F, t1975: F, t1872: F, t544: F, t2204: F, t732: F, t97: F, t1884: F, t549: F, t1891: F, t47: F, t6534: F, t6541: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t6702 = t2229 * t758;
    let t6704 = t1972 * t1975;
    let t6709 = 12.0 * t544 * t1872;
    let t6711 = 35.0 / 3.0 * t732 * t2204;
    let t6713 = 1.0 / t97 / t43;
    let t6716 = t1884 * t549;
    let t6722 = piecewise3(t44, 0.0, -8.0 / 27.0 * t6713 * t6534 + 4.0 / 3.0 * t6716 * t1891 + 4.0 / 3.0 * t47 * t6541);
    (t6702, t6704, t6709, t6711, t6713, t6716, t6722)
}
