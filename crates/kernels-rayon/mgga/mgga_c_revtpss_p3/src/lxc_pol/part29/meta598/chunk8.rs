//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2035/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2035(t100964: f64, t100975: f64, t100982: f64, t101016: f64, t101065: f64, t101093: f64, t102854: f64, t102864: f64, t102877: f64, t102888: f64, t102917: f64, t103586: f64, t1940: f64, t2403: f64, t25760: f64, t25763: f64, t25778: f64, t26425: f64, t26585: f64, t27764: f64, t27806: f64, t27817: f64, t28472: f64, t7207: f64, t7432: f64, t8020: f64) -> f64 {
    let t103778 = -3.0_f64 * t102888 * t25760 + 2.0_f64 * t28472 * t101016 + t28472 * t101065 - t1940 * t7432 * t101093 / 2.0_f64 - 3.0_f64 * t28472 * t100982 + t102877 + t1940 * t103586 * t25778 - t1940 * t26585 * t27817 + 6.0_f64 * t102864 * t27764 - 3.0_f64 / 2.0_f64 * t26425 * t100964 + t102917 + 2.0_f64 * t28472 * t100975 - t1940 * t26585 * t27806 - t1940 * t102854 * t7207 + 3.0_f64 * t2403 * t8020 * t25763;
    t103778
}
