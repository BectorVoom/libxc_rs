//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1359/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1359(t26966: f64, t26999: f64, t27042: f64, t27070: f64, t27077: f64, t28132: f64, t28137: f64, t28179: f64, t28204: f64, t7772: f64, t7788: f64, t95909: f64, t95913: f64, t97039: f64, t97051: f64, t97056: f64, t97060: f64, t97063: f64) -> f64 {
    let t97066 = 0.23214722222222222222e-2_f64 * t95909 - 0.185671721767578125e-4_f64 * t27077 * t97039 - 0.92754700520833333334e-4_f64 * t28204 * t26999 - 0.92754700520833333334e-4_f64 * t27070 * t28132 + 0.37069444444444444444e-2_f64 * t26966 * t28179 - 0.38691203703703703704e-2_f64 * t95913 + t97051 + 0.74203760416666666667e-3_f64 * t27042 * t28137 - 0.13913205078125e-3_f64 * t7772 * t97056 - t97060 - 0.34752604166666666667e-3_f64 * t7788 * t97063;
    t97066
}
