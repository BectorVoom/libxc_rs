//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1028/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1028<F: Float>(t115704: F, t115708: F, t115712: F, t115716: F, t115718: F, t115721: F, t115725: F, t115727: F, t115728: F, t115732: F, t115738: F, t117445: F, t2075: F, t24167: F, t24543: F, t24552: F, t24935: F, t31832: F, t3929: F, t510: F, t7042: F, t7220: F, t8690: F, t8840: F) -> F {
    let t117634 = -F::cast_from(2.0_f64) * t117445 * t510 - t2075 * t24543 - F::cast_from(2.0_f64) * t2075 * t24935 + t24167 * t8690 - F::cast_from(2.0_f64) * t24552 * t7042 - F::cast_from(2.0_f64) * t31832 * t7220 + t3929 * t8840 - t115704 - t115708 - t115712 + t115716 - t115718 + t115721 - t115725 - t115727 - t115728 - t115732 - t115738;
    t117634
}
