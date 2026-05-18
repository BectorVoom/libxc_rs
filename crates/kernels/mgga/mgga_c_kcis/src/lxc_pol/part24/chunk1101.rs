//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1101/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1101<F: Float>(t14668: F, t8064: F, t5036: F, t8081: F, t2189: F, t6638: F, t10498: F, t1820: F, t3330: F, t6735: F, t377: F, t6681: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29033 = F::new(4.0) * t14668 * t8064;
    let t29035 = F::new(2.0) * t5036 * t8081;
    let t29036 = t2189 * t6638;
    let t29038 = F::new(6.0) * t10498 * t29036;
    let t29039 = t8081 * t1820;
    let t29041 = F::new(4.0) * t3330 * t29039;
    let t29042 = t2189 * t6735;
    let t29044 = F::new(2.0) * t3330 * t29042;
    let t29045 = t6681 * t377;
    (t29033, t29035, t29036, t29038, t29039, t29041, t29042, t29044, t29045)
}
