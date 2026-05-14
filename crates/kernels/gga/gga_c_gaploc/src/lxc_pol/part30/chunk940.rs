//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 940/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk940<F: Float>(t10667: F, t314: F, t313: F, t11041: F, t11043: F, t11046: F, t11049: F, t11050: F, t11056: F, t11059: F, t11063: F, t11067: F, t11071: F, t11072: F, t11075: F, t11080: F, t1991: F, t317: F, t3465: F, t784: F, t797: F, t813: F) -> (F, F, F) {
    let t11083 = t314 * t10667;
    let t11084 = t313 * t11083;
    let t11087 = -t11041 + t11043 + t11046 + t11049 + 0.51123901271894332902e0 * t1991 * t11050 - t11056 - t11059 + t11063 - t11067 + t11071 - 0.30674340763136599741e1 * t813 * t11072 - 0.23833659967900284446e0 * t797 * t11075 + 0.23833659967900284446e0 * t3465 * t784 + 0.35750489951850426669e0 * t11080 * t317 + 0.35750489951850426669e0 * t11084 * t317;
    (t11083, t11084, t11087)
}
