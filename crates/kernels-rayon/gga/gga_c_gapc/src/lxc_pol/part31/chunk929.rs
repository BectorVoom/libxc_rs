//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 929/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk929(t8878: f64, t8881: f64, t8886: f64, t8889: f64, t8891: f64, t8898: f64, t8901: f64, t8904: f64, t8908: f64, t8913: f64, t8917: f64, t8919: f64, t8922: f64) -> f64 {
    let t10633 = -0.16882592796244404291e-6_f64 * t8878 - 0.20011499994481700554e-7_f64 * t8881 - 0.98481791311425691698e-7_f64 * t8886 + 0.24761136101158459626e-5_f64 * t8889 - 0.24326659074064819793e-2_f64 * t8891 - 0.75091666377929252765e-6_f64 * t8898 + 0.4637672555408563478e-4_f64 * t8901 + 0.4048307291666666667e-4_f64 * t8904 - 0.17989505234049721814e-7_f64 * t8908 + 0.23989005229605304038e-7_f64 * t8913 - 0.9004049491330348955e-7_f64 * t8917 - 0.88164651269276333518e-6_f64 * t8919 + 0.51491428373437201896e-5_f64 * t8922;
    t10633
}
