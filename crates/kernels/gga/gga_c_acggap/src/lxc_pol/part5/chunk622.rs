//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 622/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk622<F: Float>(t43: F, t40: F, t4064: F, t483: F, t803: F, t2898: F, t474: F, t34: F, t817: F, t1281: F, t1284: F, t292: F, t39: F, t4000: F, t818: F, t821: F, t824: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t4065 = t40 * t4064;
    let t4068 = t483 * t803;
    let t4069 = t40 * t4068;
    let t4070 = t2898 * t474;
    let t4073 = t817 * t34;
    let t4083 = piecewise3::<F>(t44, F::new(0.0), F::new(8.0) / F::new(27.0) * t4070 * t818 - F::new(8.0) / F::new(9.0) * t4073 * t4000 - F::new(2.0) / F::new(9.0) * t1281 * t824 + F::new(4.0) / F::new(3.0) * t292 * t821 - F::new(4.0) * t1284 * t39);
    (t4065, t4068, t4069, t4070, t4083)
}
