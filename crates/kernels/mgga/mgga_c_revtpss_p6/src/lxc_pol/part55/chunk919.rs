//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 919/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk919<F: Float>(t25931: F, t27864: F, t1955: F, t7283: F, t14224: F, t72: F, t7920: F, t686: F, t25895: F, t25878: F, t25882: F, t25893: F, t25896: F, t25921: F, t25930: F, t27837: F, t27841: F, t27846: F, t27853: F, t27858: F, t27861: F, t7295: F, t7304: F, t7926: F) -> (F, F, F, F, F) {
    let t27865 = t25931 * t27864;
    let t27868 = t1955 * t7283;
    let t27869 = t25931 * t14224;
    let t27872 = t7920 * t72;
    let t27873 = t27872 * t686;
    let t27874 = t25895 * t27873;
    let t27876 = t25878 * t27873;
    let t27879 = F::cast_from(0.4336814094102599731e0_f64) * t27837 * t7304 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t27841 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t27846 + F::cast_from(0.4336814094102599731e0_f64) * t25921 * t7926 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t27853 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t27858 - F::cast_from(0.9757440539382783019e-2_f64) * t27861 + F::cast_from(0.25702851531048074406e-1_f64) * t25882 + t25893 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t27865 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t27869 - F::cast_from(0.14456046980341999104e-1_f64) * t27874 + F::cast_from(0.25702851531048074406e-1_f64) * t27876 - F::cast_from(0.14456046980341999104e-1_f64) * t25896;
    (t27865, t27868, t27869, t27873, t27879)
}
