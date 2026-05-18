//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1001/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1001<F: Float>(t1628: F, t3714: F, t1: F, t12012: F, t544: F, t10361: F, t10363: F, t10367: F, t10369: F, t10373: F, t10377: F, t10381: F, t10384: F, t10387: F, t10394: F, t1424: F, t597: F, t9362: F, t9365: F, t9369: F) -> (F, F, F, F) {
    let t12075 = t1628 * t3714;
    let t12078 = t12012 * t1;
    let t12079 = t544 * t12078;
    let t12085 = -t10361 - t10363 - t10367 - t10369 - t10373 - t10377 + t10381 + t10384 + t10387 + F::new(0.30674340763136599741e1) * t597 * t12075 - F::new(0.39722766613167140743e-1) * t12079 * t1424 + F::new(0.38342925953920749677e0) * t9362 + F::new(0.38342925953920749677e0) * t9365 - F::new(0.85206502119823888171e-1) * t9369 + t10394;
    (t12075, t12078, t12079, t12085)
}
