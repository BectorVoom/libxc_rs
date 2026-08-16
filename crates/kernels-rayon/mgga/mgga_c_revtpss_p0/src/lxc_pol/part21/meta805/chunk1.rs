//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2929/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2929(t1058: f64, t15866: f64, t15888: f64, t11656: f64, t11961: f64, t12004: f64, t15811: f64, t1659: f64, t225: f64, t366: f64, t375: f64, t4803: f64, t4808: f64, t52977: f64, t53290: f64, t53294: f64, t53298: f64, t53300: f64) -> f64 {
    let t53302 = t15866 * t1058;
    let t53308 = t15888 * t1058;
    let t53310 = 0.22866142996303859718e-2_f64 * t11656 * t15811 - 0.28963781128651555643e-1_f64 * t12004 * t4803 + 0.2413648427387629637e-1_f64 * t12004 * t4808 - 0.45732285992607719436e-2_f64 * t53290 - t53294 - 0.53100265402527852012e-1_f64 * t1659 * t11961 * t375 + 0.14481890564325777821e-1_f64 * t53298 + 0.7622047665434619906e-3_f64 * t53300 - 0.45732285992607719436e-2_f64 * t53302 + 0.21437009059034868486e-3_f64 * t52977 * t225 * t366 * t375 + 0.42874018118069736972e-3_f64 * t53308;
    t53310
}
