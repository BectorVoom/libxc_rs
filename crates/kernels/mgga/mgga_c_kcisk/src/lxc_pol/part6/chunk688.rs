//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 688/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk688<F: Float>(t12017: F, t755: F, t10568: F, t10641: F, t1964: F, t5396: F, t5399: F, t763: F, t10690: F, t591: F, t10696: F, t10463: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t12018 = t755 * t12017;
    let t12042 = F::cast_from(0.16068111111111111111e1_f64) * t10568;
    let t12043 = F::cast_from(0.46308888888888888888e0_f64) * t10641;
    let t12058 = F::new(1.0) / t5396 / t1964;
    let t12059 = t755 * t12058;
    let t12061 = F::new(1.0) / t5399 / t763;
    let t12098 = t591 * t10690;
    let t12105 = t591 * t10696;
    let t12169 = t786 * t10463;
    (t12018, t12042, t12043, t12059, t12061, t12098, t12105, t12169)
}
