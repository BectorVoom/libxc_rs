//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1730/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1730<F: Float>(t1983: F, t22585: F, t1266: F, t1393: F, t1869: F, t1976: F, t1980: F, t22460: F, t22461: F, t22467: F, t22482: F, t22483: F, t22559: F, t22563: F, t22577: F, t22580: F, t22583: F, t2314: F, t2320: F, t2323: F, t3652: F, t3929: F, t510: F, t650: F, t6515: F, t6517: F, t652: F, t6539: F, t672: F, t6862: F, t6872: F) -> F {
    let t22587 = F::cast_from(3.0_f64) * t1983 * t22585;
    let t22588 = -F::cast_from(2.0_f64) * t1266 * t6515 + F::cast_from(2.0_f64) * t1393 * t6872 - t1869 * t3652 - F::cast_from(2.0_f64) * t1976 * t2320 + t1980 * t3929 - F::cast_from(4.0_f64) * t22461 * t672 - F::cast_from(2.0_f64) * t22483 * t652 - t22559 * t510 - F::cast_from(4.0_f64) * t2314 * t6539 - F::cast_from(4.0_f64) * t2323 * t6517 - F::cast_from(2.0_f64) * t650 * t6862 - t22460 - t22467 - t22482 - t22563 - t22577 - t22580 - t22583 + t22587;
    t22588
}
