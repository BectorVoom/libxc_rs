//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 697/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk697<F: Float>(t1021: F, t12499: F, t181: F, t197: F, t3107: F, t944: F, t955: F, t28: F, t2883: F, t14: F, t2857: F, t829: F) -> (F, F, F, F, F) {
    let t12500 = t1021 * t12499;
    let t12503 = t197 * t181;
    let t12505 = t944 * t955 * t3107;
    let t12512 = F::new(1.0) / t2883 / t28;
    let t12513 = t14 * t12512;
    let t12514 = t2857 * t829;
    (t12500, t12503, t12505, t12513, t12514)
}
