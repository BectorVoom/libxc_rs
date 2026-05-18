//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 748/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk748<F: Float>(t102: F, t8894: F, t1648: F, t1894: F, t8893: F, t1026: F, t1846: F, t637: F, t1510: F, t2982: F, t3084: F, t3131: F, t3707: F) -> (F, F, F, F, F, F) {
    let t8895 = t8894 * t102;
    let t8897 = t8895 * t1648 * t1894;
    let t8898 = t8893 * t8897;
    let t8900 = t1846 * t1026;
    let t8901 = t8900 * t637;
    let t8903 = t2982 * t1510;
    let t8904 = t3084 * t8903;
    let t8906 = t3131 * t3707;
    (t8895, t8898, t8901, t8903, t8904, t8906)
}
