//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 414/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk414<F: Float>(t1735: F, t616: F, t633: F, t663: F, t582: F, t611: F, t185: F, t1687: F, t1689: F, t1694: F, t1700: F, t1704: F) -> (F, F, F, F, F) {
    let t1737 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t616 * t1735;
    let t1739 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t633 * t663;
    let t1740 = t582 * t611;
    let t1741 = t185 * t1740;
    let t1742 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1741;
    let t1743 = F::cast_from(0.25188888888888888889e-2_f64) * t1687;
    let t1748 = -t1743 - F::cast_from(0.12594444444444444445e-2_f64) * t1689 + F::cast_from(0.12594444444444444445e-2_f64) * t1694 - F::cast_from(0.37783333333333333334e-2_f64) * t1700 + F::cast_from(0.18891666666666666667e-2_f64) * t1704;
    (t1737, t1739, t1740, t1742, t1748)
}
