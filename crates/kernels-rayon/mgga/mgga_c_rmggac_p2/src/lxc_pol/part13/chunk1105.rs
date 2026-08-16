//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1105/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1105(t40907: f64, t40872: f64, t40874: f64, t40877: f64, t40879: f64, t40881: f64, t40885: f64, t40889: f64, t40891: f64, t40895: f64, t40899: f64, t40903: f64) -> f64 {
    let t44070 = 0.21819729323396273384e0_f64 * t40907;
    let t44071 = -0.81823984962736025192e-1_f64 * t40872 - 0.40911992481368012596e-1_f64 * t40874 + 0.16364796992547205038e0_f64 * t40877 + 0.81823984962736025192e-1_f64 * t40879 + 0.40911992481368012596e-1_f64 * t40881 + 0.20455996240684006298e-1_f64 * t40885 + 0.5454932330849068346e-1_f64 * t40889 + 0.14546486215597515589e0_f64 * t40891 + 0.16364796992547205038e0_f64 * t40895 - 0.43639458646792546768e0_f64 * t40899 + 0.8182398496273602519e0_f64 * t40903 + t44070;
    t44071
}
