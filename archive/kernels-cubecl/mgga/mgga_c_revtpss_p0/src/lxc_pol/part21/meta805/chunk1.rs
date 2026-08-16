//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2929/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2929<F: Float>(t1058: F, t15866: F, t15888: F, t11656: F, t11961: F, t12004: F, t15811: F, t1659: F, t225: F, t366: F, t375: F, t4803: F, t4808: F, t52977: F, t53290: F, t53294: F, t53298: F, t53300: F) -> F {
    let t53302 = t15866 * t1058;
    let t53308 = t15888 * t1058;
    let t53310 = F::cast_from(0.22866142996303859718e-2_f64) * t11656 * t15811 - F::cast_from(0.28963781128651555643e-1_f64) * t12004 * t4803 + F::cast_from(0.2413648427387629637e-1_f64) * t12004 * t4808 - F::cast_from(0.45732285992607719436e-2_f64) * t53290 - t53294 - F::cast_from(0.53100265402527852012e-1_f64) * t1659 * t11961 * t375 + F::cast_from(0.14481890564325777821e-1_f64) * t53298 + F::cast_from(0.7622047665434619906e-3_f64) * t53300 - F::cast_from(0.45732285992607719436e-2_f64) * t53302 + F::cast_from(0.21437009059034868486e-3_f64) * t52977 * t225 * t366 * t375 + F::cast_from(0.42874018118069736972e-3_f64) * t53308;
    t53310
}
