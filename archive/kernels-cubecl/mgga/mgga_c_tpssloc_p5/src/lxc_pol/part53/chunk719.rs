//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 719/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk719<F: Float>(t533: F, t8807: F, t3701: F, t113: F, t1983: F, t2036: F, t2040: F, t2075: F, t2096: F, t510: F, t574: F, t652: F, t7042: F, t8329: F, t8607: F, t8711: F, t8718: F, t8721: F, t8774: F, t8780: F, t8805: F) -> (F, F, F) {
    let t8808 = t533 * t8807;
    let t8809 = t8808 * t3701;
    let t8811 = -t113 * t8774 + t1983 * t8805 - t1983 * t8809 - F::cast_from(2.0_f64) * t2036 * t2075 - F::cast_from(4.0_f64) * t2040 * t7042 + F::cast_from(2.0_f64) * t2096 * t8607 - t510 * t8711 - F::cast_from(2.0_f64) * t510 * t8718 + t574 * t8780 - F::cast_from(4.0_f64) * t652 * t8721 - t8329;
    (t8808, t8809, t8811)
}
