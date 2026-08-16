//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1433/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1433<F: Float>(t22237: F, t4869: F, t78242: F, t78247: F, t78250: F, t78254: F, t78281: F, t78283: F, t78286: F, t78291: F, t78294: F, t78296: F, t78298: F, t78302: F) -> (F, F) {
    let t78304 = F::cast_from(0.4101607543286562663e4_f64) * t4869 * t22237;
    let t78305 = t78242 - t78247 + t78250 + t78254 - t78281 - t78283 + t78286 - t78291 - t78294 + t78296 - t78298 + t78302 - t78304;
    (t78304, t78305)
}
