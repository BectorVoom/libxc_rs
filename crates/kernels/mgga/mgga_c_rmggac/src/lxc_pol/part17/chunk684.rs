//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 684/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk684<F: Float>(t1364: F, t9855: F, t2402: F, t551: F, t739: F, t2295: F, t6355: F, t1704: F, t27: F, t649: F, t7282: F, t570: F, t8800: F) -> (F, F, F, F, F, F, F) {
    let t9856 = t1364 * t9855;
    let t9857 = F::cast_from(0.23948483403727617128e0_f64) * t9856;
    let t9858 = t2402 * t551;
    let t9859 = t739 * t9858;
    let t9860 = F::cast_from(0.11974241701863808564e0_f64) * t9859;
    let t9861 = t6355 * t2295;
    let t9862 = F::cast_from(0.5987120850931904282e-1_f64) * t9861;
    let t9864 = t27 * t649 * t1704;
    let t9865 = t7282 * t9864;
    let t9866 = F::cast_from(0.20455996240684006296e-1_f64) * t9865;
    let t9867 = t8800 * t570;
    (t9857, t9858, t9860, t9862, t9864, t9866, t9867)
}
