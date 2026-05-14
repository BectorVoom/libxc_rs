//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 626/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk626<F: Float>(t1517: F, t1650: F, t5987: F, t4230: F, t6281: F, t1518: F, t6284: F, t509: F, t7190: F, t1153: F, t1991: F, t1995: F, t2018: F, t368: F, t4202: F, t4213: F, t545: F, t562: F, t5966: F, t5985: F, t7233: F, t7237: F, t7241: F, t7245: F, t7249: F, t7341: F, t7361: F, t86: F) -> (F, F, F, F, F) {
    let t7365 = t1517 * t5987 * t1650;
    let t7369 = t1517 * t4230 * t6281;
    let t7373 = t1517 * t1518 * t6284;
    let t7376 = t509 * t7190;
    let t7380 = 0.619125e-2 * t7341 * t545 + 0.1857375e-1 * t2018 * t1991 - 0.123825e-1 * t2018 * t1995 + 0.46434375e-2 * t562 * t7233 - 0.1857375e-1 * t4202 * t7237 + 0.9286875e-2 * t562 * t7241 + 0.123825e-1 * t562 * t7245 - 0.619125e-2 * t562 * t7249 + t4213 - 0.35374814814814814814e-1 * t5966 - 0.53062222222222222222e-1 * t5985 - 0.44218518518518518518e-1 * t1153 * t7361 - 0.53062222222222222222e-1 * t1153 * t7365 + 0.53062222222222222222e-1 * t1153 * t7369 - 0.26531111111111111111e-1 * t1153 * t7373 - 0.39796666666666666666e-1 * t86 * t368 * t7376;
    (t7365, t7369, t7373, t7376, t7380)
}
