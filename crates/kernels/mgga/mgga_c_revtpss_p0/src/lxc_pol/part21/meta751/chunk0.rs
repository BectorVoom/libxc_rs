//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2628/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2628<F: Float>(t47133: F, t47135: F, t13665: F, t9572: F, t1320: F, t13680: F, t47145: F, t47147: F, t47149: F, t3863: F, t5569: F, t3860: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48322 = F::cast_from(0.32530743900905219526e-1_f64) * t47133;
    let t48323 = F::cast_from(0.65061487801810439052e-1_f64) * t47135;
    let t48324 = t13665 * t9572;
    let t48325 = F::cast_from(0.32530743900905219526e-1_f64) * t48324;
    let t48326 = t1320 * t13680;
    let t48327 = F::new(24.0) * t48326;
    let t48328 = F::cast_from(0.51947577317044391277e2_f64) * t47145;
    let t48329 = F::cast_from(0.30762056574649219973e4_f64) * t47147;
    let t48330 = F::new(12.0) * t47149;
    let t48331 = t3863 * t5569;
    let t48332 = F::new(96.0) * t48331;
    let t48333 = t3860 * t5569;
    (t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332, t48333)
}
