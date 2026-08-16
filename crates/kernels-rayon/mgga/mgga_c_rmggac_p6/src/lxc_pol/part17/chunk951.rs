//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 951/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk951(t1704: f64, t2084: f64, t27: f64, t7282: f64, t39667: f64, t39679: f64, t39694: f64, t39698: f64, t39702: f64, t45757: f64, t45759: f64, t45763: f64, t45767: f64, t45769: f64, t45775: f64, t45777: f64, t45779: f64, t45781: f64, t45788: f64, t4985: f64, t739: f64, t8960: f64) -> f64 {
    let t45794 = t7282 * t27 * t2084 * t1704;
    let t45796 = -0.25538759935978703639e-4_f64 * t45757 - 0.85129199786595678796e-5_f64 * t45759 + 0.85129199786595678796e-5_f64 * t45763 - 0.53205749866622299248e-5_f64 * t45767 - 0.59871208509319042821e-1_f64 * t739 * t45769 + 0.54549323308490683458e-1_f64 * t39667 - 0.27274661654245341728e-1_f64 * t45775 + 0.25538759935978703638e-4_f64 * t45777 + 0.25538759935978703638e-4_f64 * t45779 + t39679 - 0.76616279807936110914e-4_f64 * t45781 + 0.21819729323396273383e0_f64 * t39694 + 0.54549323308490683457e-1_f64 * t39698 - t39702 - 0.25538759935978703638e-4_f64 * t45788 + 0.11974241701863808564e0_f64 * t4985 * t8960 - 0.54549323308490683456e-1_f64 * t45794;
    t45796
}
