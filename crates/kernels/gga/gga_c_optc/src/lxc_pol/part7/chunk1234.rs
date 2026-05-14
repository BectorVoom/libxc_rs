//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1234/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1234<F: Float>(t3156: F, t7878: F, t1133: F, t2586: F, t8952: F, t2849: F, t381: F, t26336: F, t22035: F, t894: F, t1: F, t1111: F, t11596: F, t26276: F, t26287: F, t26317: F, t26322: F, t27148: F, t27153: F, t27158: F, t27167: F, t27174: F, t27175: F, t3245: F, t4289: F, t438: F, t450: F, t8966: F, t8968: F, t8973: F) -> (F, F, F, F) {
    let t27181 = t7878 * t3156;
    let t27182 = t1133 * t27181;
    let t27184 = t2586 * t8952;
    let t27185 = t1133 * t27184;
    let t27188 = 1.0 / t381 / t2849;
    let t27189 = t27188 * t26336;
    let t27191 = t894 * t27189 * t22035;
    let t27194 = -t1111 * t4289 * t26287 / 6.0 + 0.36629113921839320675e2 * t8966 * t8968 * t27148 - 0.73258227843678641351e2 * t8973 * t8968 * t27153 + t27158 / 54.0 - t1111 * t3245 * t26317 / 36.0 + t1111 * t4289 * t26322 / 54.0 - t27167 / 36.0 + 7.0 / 108.0 * t1111 * t11596 * t26276 - 0.23456682646837756387e4 * t27174 * t450 * t27175 * t1 * t438 - 0.24147670804968771818e-2 * t27182 + 0.21464596271083352727e-1 * t27185 + 0.2951381987273961e-1 * t1133 * t27191;
    (t27181, t27184, t27191, t27194)
}
