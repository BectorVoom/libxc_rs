//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1204/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1204<F: Float>(t17381: F, t20662: F, t20685: F, t20834: F, t20837: F, t20849: F, t20892: F, t20895: F, t20898: F, t20900: F, t20902: F, t20904: F, t20905: F, t20908: F, t20913: F, t20916: F, t5883: F, t5887: F, t5894: F, t721: F) -> F {
    let t20917 = -t20662 - F::new(0.31168546390226634765e3) * t20834 * t5894 + F::new(0.30762056574649219974e4) * t20837 * t17381 * t721 + t20685 - F::new(0.19751673498613801407e-1) * t20849 - t20892 + t20895 - t20898 - t20900 - t20902 - t20904 + F::new(18.0) * t20905 * t5883 - F::new(0.57895126195293126243e3) * t20908 * t5887 + t20913 - t20916;
    t20917
}
