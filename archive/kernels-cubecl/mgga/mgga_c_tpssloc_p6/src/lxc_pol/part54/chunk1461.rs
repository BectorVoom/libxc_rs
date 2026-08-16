//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1461/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1461<F: Float>(t1437: F, t31860: F, t32343: F, t8513: F, t117480: F, t1433: F, t8663: F, t63: F, t641: F, t116082: F, t116124: F, t117483: F, t117499: F, t117516: F, t117518: F, t117527: F, t122960: F, t122964: F, t122988: F, t123001: F, t31857: F, t31868: F, t32328: F, t32338: F, t32340: F, t33669: F, t33677: F, t34122: F, t34132: F, t4017: F, t4021: F, t8824: F, t8825: F) -> F {
    let t124834 = t31860 * t8513 * t32343 * t1437;
    let t124838 = t8663 * t8513 * t117480 * t1433;
    let t124844 = t641 * t63;
    let t124860 = F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t117483 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t122988 * t32328 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t33669 * t32340 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t116124 * t34122 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t116082 * t34122 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t31860 * t8513 * t8824 * t4021 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t123001 * t32328 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t33677 * t32340 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t124834 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t124838 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31857 * t34132 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31868 * t34132 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8663 * t8513 * t124844 * t1433 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8663 * t8513 * t32338 * t4017 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t117499 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t117516 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t117518 - t117527 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t122960 * t8825 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t122964 * t8825;
    t124860
}
