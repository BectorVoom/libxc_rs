//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 699/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk699<F: Float>(t515: F, t9486: F, t235: F, t2475: F, t275: F, t118: F, t305: F, t8889: F, t8891: F, t8893: F, t8895: F, t8897: F, t8899: F, t8903: F, t8907: F, t8909: F, t8911: F, t8913: F, t8917: F, t9427: F, t9437: F) -> (F, F, F, F) {
    let t9487 = t515 * t9486;
    let t9488 = t235 * t9487;
    let t9499 = t275 * t2475;
    let t9518 = -F::cast_from(0.40911992481368012596e-1_f64) * t8889 + F::cast_from(0.81823984962736025192e-1_f64) * t8891 + F::cast_from(0.20455996240684006298e-1_f64) * t8893 + F::cast_from(0.8182398496273602519e-1_f64) * t8895 - F::cast_from(0.13637330827122670865e0_f64) * t8897 - F::cast_from(0.2727466165424534173e-1_f64) * t8899 + F::cast_from(0.20455996240684006298e-1_f64) * t8903 - F::cast_from(0.2727466165424534173e-1_f64) * t8907 - F::cast_from(0.13637330827122670865e-1_f64) * t8909 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t9437 - F::cast_from(0.39914139006212695214e-1_f64) * t118 * t9427 + F::cast_from(0.54549323308490683461e-1_f64) * t8911 - F::cast_from(0.72732431077987577947e-1_f64) * t8913 - F::cast_from(0.18183107769496894487e-1_f64) * t8917;
    (t9487, t9488, t9499, t9518)
}
