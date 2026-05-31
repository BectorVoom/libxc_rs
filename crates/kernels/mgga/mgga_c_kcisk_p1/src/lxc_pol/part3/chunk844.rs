//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 844/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk844<F: Float>(t333: F, t3724: F, t12884: F, t12885: F, t3716: F, t3722: F, t5794: F, t1210: F, t3696: F, t4475: F, t1171: F, t3631: F) -> (F, F, F, F, F) {
    let t12888 = F::cast_from(1.0_f64) / t3724 / t333;
    let t12889 = t12884 * t12885 * t12888;
    let t12893 = t3722 * t3716 * t5794;
    let t12896 = t3696 * t1210;
    let t12897 = t12896 * t4475;
    let t12900 = t3631 * t1171;
    (t12888, t12889, t12893, t12897, t12900)
}
