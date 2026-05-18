//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1144/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1144<F: Float>(t2321: F, t28438: F, t9285: F, t2487: F, t6985: F, t9278: F, t18067: F, t9558: F, t20513: F, t2365: F, t4391: F, t20521: F) -> (F, F, F, F, F, F) {
    let t30733 = t28438 * t2321;
    let t30735 = F::new(0.11916829983950142223e0) * t9285 * t30733;
    let t30751 = t2487 * t6985 * t9278;
    let t30754 = F::new(0.11916829983950142223e0) * t18067 * t9558;
    let t30757 = F::new(0.11916829983950142223e0) * t4391 * t2365 * t20513;
    let t30760 = F::new(0.59584149919750711116e-1) * t4391 * t2365 * t20521;
    (t30733, t30735, t30751, t30754, t30757, t30760)
}
