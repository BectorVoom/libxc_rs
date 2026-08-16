//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1066/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1066(t38552: f64, t38554: f64, t38556: f64, t42665: f64, t44821: f64, t44825: f64, t44828: f64, t44831: f64, t44835: f64, t44838: f64, t44841: f64, t44844: f64, t44847: f64, t44850: f64, t44854: f64, t44857: f64, t44860: f64) -> f64 {
    let t48212 = -0.1440846329149835838e-2_f64 * t44821 + 0.12195059916630011325e-2_f64 * t38552 - 0.1440846329149835838e-2_f64 * t44825 - 0.1440846329149835838e-2_f64 * t44828 + 0.12195059916630011325e-2_f64 * t38554 + 0.3842256877732895568e-2_f64 * t44831 - 0.72042316457491791901e-3_f64 * t44835 - 0.72042316457491791901e-3_f64 * t44838 - 0.72042316457491791901e-3_f64 * t44841 - 0.1440846329149835838e-2_f64 * t44844 - 0.1440846329149835838e-2_f64 * t44847 - 0.1440846329149835838e-2_f64 * t44850 - 0.14088275218353950416e-1_f64 * t38556 + 0.3842256877732895568e-2_f64 * t44854 - 0.30487649791575028312e-3_f64 * t44857 - 0.72042316457491791901e-3_f64 * t44860 - t42665;
    t48212
}
