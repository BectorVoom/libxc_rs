//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1096/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1096(t1347: f64, t2475: f64, t41828: f64, t41882: f64, t41884: f64, t1550: f64, t36680: f64, t36715: f64, t38047: f64, t41834: f64, t41836: f64, t41838: f64, t41846: f64, t41848: f64, t41850: f64, t41863: f64, t41865: f64, t41887: f64, t5207: f64, t699: f64) -> f64 {
    let t43877 = t1347 * t2475;
    let t43878 = 0.39726959900411316772e-4_f64 * t41828;
    let t43891 = 0.39726959900411316772e-4_f64 * t41882;
    let t43892 = 0.39726959900411316772e-4_f64 * t41884;
    let t43895 = t43877 - t43878 + 0.5107751987195740728e-4_f64 * t41834 - 0.16364796992547205038e0_f64 * t41836 - 0.40911992481368012596e-1_f64 * t41838 - 0.11974241701863808564e0_f64 * t1550 * t699 * t5207 - 0.47896966807455234256e0_f64 * t36680 - 0.1064114997332445985e-4_f64 * t41846 - 0.5987120850931904282e-1_f64 * t41848 - 0.11974241701863808564e0_f64 * t41850 - 0.85129199786595678799e-5_f64 * t41863 - 0.1702583995731913576e-4_f64 * t41865 - t38047 + t43891 + t43892 + 0.2727466165424534173e-1_f64 * t41887 - 0.10909864661698136692e0_f64 * t36715;
    t43895
}
