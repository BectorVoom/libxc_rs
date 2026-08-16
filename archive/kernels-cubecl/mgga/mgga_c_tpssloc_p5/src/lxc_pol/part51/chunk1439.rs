//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1439/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1439<F: Float>(t120606: F, t120607: F, t120611: F, t120612: F, t120616: F, t120621: F, t122377: F, t122384: F, t122390: F, t122394: F, t26996: F, t27068: F, t31642: F, t33301: F, t3758: F, t5321: F, t6958: F, t6963: F) -> F {
    let t122396 = -t120606 + t120607 - F::cast_from(0.16449340668482264365e-1_f64) * t122377 + F::cast_from(2.0_f64) * t6958 * t26996 + F::cast_from(2.0_f64) * t3758 * t33301 - F::cast_from(0.16449340668482264365e-1_f64) * t122384 + t120611 + t120612 - t120616 - t5321 * t31642 + F::cast_from(2.0_f64) * t27068 * t6963 + F::cast_from(0.41123351671205660912e-2_f64) * t122390 - t120621 - F::cast_from(0.3289868133696452873e-1_f64) * t122394;
    t122396
}
