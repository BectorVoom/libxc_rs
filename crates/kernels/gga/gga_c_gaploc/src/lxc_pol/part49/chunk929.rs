//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 929/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk929<F: Float>(t13740: F, t484: F, t42844: F, t42845: F, t42847: F, t42850: F, t42852: F, t47024: F, t47028: F, t47032: F, t47036: F, t47040: F, t11977: F, t2268: F, t6763: F, t1063: F, t6750: F) -> (F, F, F) {
    let t47042 = t484 * t13740;
    let t47044 = -t42844 + 0.56910013271352299198e-1 * t47024 + 0.56910013271352299198e-1 * t47028 - t42845 + t42847 + t42850 - 0.31616674039640166221e-2 * t47032 - 0.63233348079280332442e-2 * t42852 + 0.11856252764865062333e-2 * t47036 + 0.28455006635676149599e-1 * t47040 + 0.15808337019820083111e-2 * t47042;
    let t47047 = t2268 * t11977 * t6763;
    let t47050 = t1063 * t11977 * t6750;
    (t47044, t47047, t47050)
}
