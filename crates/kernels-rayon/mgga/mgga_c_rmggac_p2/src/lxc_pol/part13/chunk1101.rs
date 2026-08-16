//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1101/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1101(t1356: f64, t29838: f64, t36893: f64, t38107: f64, t42011: f64, t42032: f64, t42034: f64, t42042: f64, t42050: f64, t42055: f64, t42059: f64, t43971: f64, t43975: f64, t43978: f64, t43979: f64, t43981: f64, t43987: f64, t43990: f64, t4985: f64, t8281: f64) -> f64 {
    let t43993 = 0.23942587439980034662e-4_f64 * t42011 + 0.11974241701863808564e0_f64 * t4985 * t8281 + 0.79828278012425390428e-1_f64 * t1356 * t43971 - 0.11974241701863808564e0_f64 * t1356 * t43975 - t43978 - t43979 - 0.39726959900411316772e-4_f64 * t36893 + 0.95793933614910468512e0_f64 * t29838 * t43981 + 0.30487649791575028312e-3_f64 * t42032 - 0.47896966807455234256e0_f64 * t42034 + 0.60975299583150056624e-3_f64 * t42042 + t43987 - 0.5107751987195740728e-4_f64 * t42050 + 0.5107751987195740728e-4_f64 * t42055 + t43990 + 0.5987120850931904282e-1_f64 * t42059 - 0.4726e1_f64 * t38107;
    t43993
}
