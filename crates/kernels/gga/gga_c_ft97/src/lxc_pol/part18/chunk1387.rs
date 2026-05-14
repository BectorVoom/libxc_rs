//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1387/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1387<F: Float>(t1882: F, t26838: F, t12001: F, t27273: F, t26947: F, t6701: F, t8232: F, t104525: F, t12645: F, t12650: F, t12945: F, t12950: F, t12968: F, t13140: F, t144: F, t1901: F, t2185: F, t23455: F, t23505: F, t23549: F, t26928: F, t3455: F, t3578: F, t3590: F, t446: F, t49414: F, t50773: F, t574: F, t5842: F, t5942: F, t95521: F, t96215: F) -> (F,) {
    let t107477 = 2.0 / 9.0 * t1882 * t26838;
    let t107478 = t12001 * t27273;
    let t107499 = 2.0 / 9.0 * t1882 * t26947;
    let t107519 = t8232 * t6701;
    let t107521 = -t107477 + 22.0 / 27.0 * t107478 - 4.0 / 3.0 * t1901 * t49414 * t26928 - 4.0 / 3.0 * t1901 * t12968 * t5942 * t12645 - 2.0 / 3.0 * t1901 * t12968 * t5942 * t12945 - 4.0 / 3.0 * t1901 * t13140 * t95521 * t3455 - 2.0 / 9.0 * t1901 * t50773 * t23549 - t107499 - 4.0 / 3.0 * t1901 * t13140 * t23455 * t12650 - 2.0 / 3.0 * t1901 * t13140 * t23455 * t12950 - 2.0 / 3.0 * t446 * t2185 * t3578 * t23505 - t96215 - 2.0 / 3.0 * t446 * t574 * t3590 * t5842 - 2.0 / 3.0 * t446 * t144 * t104525 + 4.0 / 27.0 * t107519;
    (t107521,)
}
