//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1088/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1088(t47825: f64, t7717: f64, t1707: f64, t2124: f64, t42024: f64, t42027: f64, t42035: f64, t42042: f64, t42044: f64, t43990: f64, t47795: f64, t47797: f64, t47800: f64, t47802: f64, t47804: f64, t47809: f64, t47814: f64, t47816: f64, t47818: f64, t47821: f64, t903: f64) -> f64 {
    let t47826 = t7717 * t47825;
    let t47828 = 0.35922725105591425692e0_f64 * t903 * t2124 * t1707 - t42024 - t42027 - 0.5987120850931904282e-1_f64 * t47795 - t42035 + 0.8980681276397856423e-1_f64 * t47797 + 0.60975299583150056628e-3_f64 * t42042 - 0.20455996240684006296e-1_f64 * t47800 - 0.81823984962736025184e-1_f64 * t47802 - 0.20455996240684006296e-1_f64 * t47804 - 0.17025839957319135759e-4_f64 * t47809 + 0.85129199786595678796e-5_f64 * t47814 + 0.1064114997332445985e-4_f64 * t47816 + 2.0_f64 * t47818 + 0.59590439850616975158e-4_f64 * t42044 + 0.19863479950205658386e-4_f64 * t47821 + 0.1064114997332445985e-4_f64 * t47826 + t43990;
    t47828
}
