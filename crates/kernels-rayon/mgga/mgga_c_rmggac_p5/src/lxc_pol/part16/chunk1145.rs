//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1145/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1145(t2265: f64, t6624: f64, t10367: f64, t275: f64, t47874: f64, t47876: f64, t47881: f64, t47883: f64, t47885: f64, t47887: f64, t47889: f64, t47891: f64, t47898: f64, t47903: f64, t47908: f64, t47913: f64, t47918: f64, t47923: f64, t47931: f64, t4985: f64, t9352: f64) -> f64 {
    let t49736 = t6624 * t2265;
    let t49738 = t275 * t10367;
    let t49747 = -0.1702583995731913576e-4_f64 * t47874 - 0.638468998399467591e-4_f64 * t47876 + 0.85129199786595678799e-5_f64 * t47881 - 0.85129199786595678799e-5_f64 * t47883 - 0.20455996240684006298e-1_f64 * t47885 + 0.2727466165424534173e-1_f64 * t47887 + 0.13637330827122670865e-1_f64 * t47889 + 0.40911992481368012596e-1_f64 * t47891 + 0.11974241701863808564e0_f64 * t4985 * t9352 - 0.2363e1_f64 * t49736 + 2.0_f64 * t49738 + 0.638468998399467591e-4_f64 * t47898 - 0.1276937996798935182e-3_f64 * t47903 + 0.1915406995198402773e-3_f64 * t47908 + 0.638468998399467591e-4_f64 * t47913 - 0.638468998399467591e-4_f64 * t47918 + 0.5107751987195740728e-4_f64 * t47923 - 0.35922725105591425692e0_f64 * t47931;
    t49747
}
