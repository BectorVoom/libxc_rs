//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 651/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk651<F: Float>(t2563: F, t2567: F, t1935: F, t8973: F, t9017: F, t9021: F, t9023: F, t9025: F, t9027: F, t9031: F, t9033: F, t9037: F, t9039: F, t9041: F) -> (F, F, F) {
    let t9043 = t2567 * t2563;
    let t9044 = t1935 * t9043;
    let t9046 = t8973 / F::cast_from(256.0_f64) + t9017 / F::cast_from(16.0_f64) - t9021 / F::cast_from(72.0_f64) + t9023 / F::cast_from(128.0_f64) - t9025 / F::cast_from(3.0_f64) + t9027 / F::cast_from(12.0_f64) - t9031 / F::cast_from(16.0_f64) - t9033 / F::cast_from(8.0_f64) + t9037 / F::cast_from(24.0_f64) + t9039 / F::cast_from(24.0_f64) - t9041 / F::cast_from(96.0_f64) + t9044 / F::cast_from(3.0_f64);
    (t9043, t9044, t9046)
}
