//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 990/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk990(t40907: f64, t36250: f64, t38565: f64, t39693: f64, t7785: f64, t35824: f64, t39045: f64, t40877: f64, t40879: f64, t40881: f64, t40885: f64, t40889: f64, t40891: f64, t40895: f64, t40899: f64, t40903: f64) -> f64 {
    let t40908 = 0.10909864661698136691e0_f64 * t40907;
    let t40909 = t36250 * t38565;
    let t40911 = t7785 * t39693;
    let t40913 = t35824 * t39045;
    let t40915 = 0.81823984962736025184e-1_f64 * t40877 + 0.40911992481368012592e-1_f64 * t40879 + 0.20455996240684006296e-1_f64 * t40881 + 0.10227998120342003148e-1_f64 * t40885 + 0.27274661654245341728e-1_f64 * t40889 + 0.72732431077987577942e-1_f64 * t40891 + 0.81823984962736025184e-1_f64 * t40895 - 0.21819729323396273382e0_f64 * t40899 + 0.40911992481368012592e0_f64 * t40903 + t40908 - 0.20455996240684006296e0_f64 * t40909 - 0.21819729323396273382e0_f64 * t40911 - 0.20455996240684006296e-1_f64 * t40913;
    t40915
}
