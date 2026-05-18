//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1039/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1039<F: Float>(t1291: F, t1293: F, t403: F, t4536: F, t18664: F, t1253: F, t18515: F, t404: F, t4509: F, t1214: F, t1318: F, t1273: F, t1286: F, t395: F) -> (F, F, F, F, F, F) {
    let t18850 = F::new(0.64327297288604419288e2) * t1291 * t4536 * t1293 * t403;
    let t18853 = F::new(0.48245472966453314466e2) * t1291 * t18664 * t1293;
    let t18854 = t1253 * t1253;
    let t18863 = F::new(24.0) * t4509 * t18515 * t404;
    let t18865 = F::new(1.0) / t1318 / t1214;
    let t18885 = F::new(0.4274e0) * t395 * t1273 * t403 * t1286;
    (t18850, t18853, t18854, t18863, t18865, t18885)
}
