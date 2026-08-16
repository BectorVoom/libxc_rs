//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2282/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2282<F: Float>(t90472: F, t1385: F, t1992: F, t22635: F, t3886: F, t5353: F, t3888: F, t55118: F, t1799: F, t22633: F, t80663: F, t80671: F) -> (F, F, F, F, F, F) {
    let t90473 = F::cast_from(0.76763589786250567036e-1_f64) * t90472;
    let t90477 = t1992 * t22635 * t3886 * t5353 * t1385;
    let t90485 = t1992 * t22635 * t55118 * t3888;
    let t90488 = t3886 * t1799;
    let t90491 = t22633 * t22635 * t90488 * t3888;
    let t90493 = F::cast_from(0.12793931631041761173e0_f64) * t80663;
    let t90496 = F::cast_from(0.10417915756705434098e0_f64) * t80671;
    (t90473, t90477, t90485, t90491, t90493, t90496)
}
