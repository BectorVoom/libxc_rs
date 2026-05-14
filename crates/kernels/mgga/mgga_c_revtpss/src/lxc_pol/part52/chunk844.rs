//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 844/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk844<F: Float>(t1444: F, t1883: F, t25931: F, t1955: F, t7283: F, t14224: F, t72: F, t7920: F, t686: F, t25895: F, t25878: F, t25882: F, t25893: F, t25896: F, t25921: F, t25930: F, t27837: F, t27841: F, t27846: F, t27853: F, t27858: F, t27861: F, t7295: F, t7304: F, t7926: F) -> (F, F, F, F, F, F) {
    let t27864 = t1883 * t1444;
    let t27865 = t25931 * t27864;
    let t27868 = t1955 * t7283;
    let t27869 = t25931 * t14224;
    let t27872 = t7920 * t72;
    let t27873 = t27872 * t686;
    let t27874 = t25895 * t27873;
    let t27876 = t25878 * t27873;
    let t27879 = 0.4336814094102599731e0 * t27837 * t7304 - 0.26020884564615598386e1 * t7295 * t27841 + 0.4336814094102599731e0 * t7295 * t27846 + 0.4336814094102599731e0 * t25921 * t7926 + 0.4336814094102599731e0 * t7295 * t27853 + 0.4336814094102599731e0 * t7295 * t27858 - 0.9757440539382783019e-2 * t27861 + 0.25702851531048074406e-1 * t25882 + t25893 - 0.8673628188205199462e0 * t25930 * t27865 + 0.4336814094102599731e0 * t27868 * t27869 - 0.14456046980341999104e-1 * t27874 + 0.25702851531048074406e-1 * t27876 - 0.14456046980341999104e-1 * t25896;
    (t27864, t27865, t27868, t27869, t27873, t27879)
}
