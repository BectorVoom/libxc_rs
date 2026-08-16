//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 457/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk457(t118: f64, t305: f64, t8889: f64, t8891: f64, t8893: f64, t8895: f64, t8897: f64, t8899: f64, t8903: f64, t8907: f64, t8909: f64, t8911: f64, t8913: f64, t8917: f64, t9427: f64, t9437: f64) -> f64 {
    let t9518 = -0.40911992481368012596e-1_f64 * t8889 + 0.81823984962736025192e-1_f64 * t8891 + 0.20455996240684006298e-1_f64 * t8893 + 0.8182398496273602519e-1_f64 * t8895 - 0.13637330827122670865e0_f64 * t8897 - 0.2727466165424534173e-1_f64 * t8899 + 0.20455996240684006298e-1_f64 * t8903 - 0.2727466165424534173e-1_f64 * t8907 - 0.13637330827122670865e-1_f64 * t8909 + 0.59871208509319042821e-1_f64 * t305 * t9437 - 0.39914139006212695214e-1_f64 * t118 * t9427 + 0.54549323308490683461e-1_f64 * t8911 - 0.72732431077987577947e-1_f64 * t8913 - 0.18183107769496894487e-1_f64 * t8917;
    t9518
}
