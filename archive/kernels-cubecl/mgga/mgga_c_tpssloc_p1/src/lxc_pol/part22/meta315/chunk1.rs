//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1495/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1495<F: Float>(t3572: F, t5002: F, t3523: F, t5005: F, t5019: F, t5024: F, t11147: F, t11778: F, t3490: F, t4993: F, t248: F, t3521: F, t4733: F) -> (F, F, F, F, F, F, F) {
    let t15446 = t5002 * t3572 / F::cast_from(2304.0_f64);
    let t15448 = t5005 * t3523 / F::cast_from(3456.0_f64);
    let t15450 = t5019 * t3572 / F::cast_from(432.0_f64);
    let t15452 = t5024 * t3523 / F::cast_from(648.0_f64);
    let t15453 = t11778 * t11147;
    let t15484 = t3490 * t4993 / F::cast_from(3456.0_f64);
    let t15486 = t248 * t3521 * t4733;
    (t15446, t15448, t15450, t15452, t15453, t15484, t15486)
}
