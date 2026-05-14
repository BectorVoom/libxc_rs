//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1063/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1063<F: Float>(t31965: F, t2321: F, t8289: F, t882: F, t1217: F, t3344: F, t10273: F, t4141: F, t20369: F, t2268: F, t24139: F, t8124: F, t3808: F, t1358: F, t3394: F, t488: F, t6540: F) -> (F, F, F, F, F, F, F) {
    let t31966 = 0.31616674039640166222e-2 * t31965;
    let t31968 = t882 * t8289 * t2321;
    let t31969 = 0.11856252764865062333e-2 * t31968;
    let t31973 = t1217 * t3344;
    let t31974 = 0.36886119712913527259e-2 * t31973;
    let t31984 = 0.31616674039640166222e-2 * t4141 * t10273;
    let t31988 = 0.68292015925622759036e0 * t2268 * t24139 * t8124 * t20369;
    let t31990 = 0.63233348079280332442e-2 * t3808 * t10273;
    let t31994 = 0.63233348079280332442e-2 * t1358 * t6540 * t3394 * t488;
    (t31966, t31969, t31974, t31984, t31988, t31990, t31994)
}
