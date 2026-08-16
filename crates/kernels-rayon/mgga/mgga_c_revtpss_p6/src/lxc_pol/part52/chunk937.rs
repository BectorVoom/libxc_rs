//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 937/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk937(t1955: f64, t7283: f64, t14224: f64, t25931: f64, t72: f64, t7920: f64, t686: f64, t25895: f64, t25878: f64, t25882: f64, t25893: f64, t25896: f64, t25921: f64, t25930: f64, t27837: f64, t27841: f64, t27846: f64, t27853: f64, t27858: f64, t27861: f64, t27865: f64, t7295: f64, t7304: f64, t7926: f64) -> (f64, f64, f64, f64) {
    let t27868 = t1955 * t7283;
    let t27869 = t25931 * t14224;
    let t27872 = t7920 * t72;
    let t27873 = t27872 * t686;
    let t27874 = t25895 * t27873;
    let t27876 = t25878 * t27873;
    let t27879 = 0.4336814094102599731e0_f64 * t27837 * t7304 - 0.26020884564615598386e1_f64 * t7295 * t27841 + 0.4336814094102599731e0_f64 * t7295 * t27846 + 0.4336814094102599731e0_f64 * t25921 * t7926 + 0.4336814094102599731e0_f64 * t7295 * t27853 + 0.4336814094102599731e0_f64 * t7295 * t27858 - 0.9757440539382783019e-2_f64 * t27861 + 0.25702851531048074406e-1_f64 * t25882 + t25893 - 0.8673628188205199462e0_f64 * t25930 * t27865 + 0.4336814094102599731e0_f64 * t27868 * t27869 - 0.14456046980341999104e-1_f64 * t27874 + 0.25702851531048074406e-1_f64 * t27876 - 0.14456046980341999104e-1_f64 * t25896;
    (t27868, t27869, t27873, t27879)
}
