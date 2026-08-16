//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1261/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1261(t1020: f64, t1133: f64, t1135: f64, t1137: f64, t12286: f64, t12288: f64, t12895: f64, t12897: f64, t12899: f64, t12901: f64, t12903: f64, t2410: f64, t2956: f64, t333: f64, t3534: f64, t3538: f64, t3542: f64, t3761: f64, t3765: f64, t44685: f64, t839: f64, t9707: f64) -> f64 {
    let t44811 = -0.3831420472412e2_f64 * t3534 * t2956 - 0.3831420472412e2_f64 * t1133 * t9707 + 0.3101306810232e2_f64 * t12286 * t1020 + 0.3101306810232e2_f64 * t3761 * t2410 + 0.1550653405116e2_f64 * t3538 * t2956 + 0.1550653405116e2_f64 * t1135 * t9707 - 0.4355305902528e1_f64 * t12288 * t1020 - 0.4355305902528e1_f64 * t3765 * t2410 - 0.2177652951264e1_f64 * t3542 * t2956 - 0.2177652951264e1_f64 * t1137 * t9707 - 0.9214113627294e1_f64 * t12895 * t839 + 0.367387230261e2_f64 * t12897 * t839 - 0.3831420472412e2_f64 * t12899 * t839 + 0.1550653405116e2_f64 * t12901 * t839 - 0.2177652951264e1_f64 * t12903 * t839 - 0.8704e0_f64 * t333 * t44685;
    t44811
}
