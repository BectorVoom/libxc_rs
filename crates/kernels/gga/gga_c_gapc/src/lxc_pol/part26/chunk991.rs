//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 991/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk991<F: Float>(t11808: F, t30187: F, t3131: F, t5658: F, t1084: F, t29568: F, t11781: F, t3368: F, t34036: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F, t34054: F) -> (F, F) {
    let t34056 = t11808 * t30187;
    let t34058 = t3131 * t5658;
    let t34060 = t1084 * t34058 * t29568;
    let t34062 = t11781 * t3368;
    let t34064 = -0.58333107277199074076e-4 * t34036 + 0.57970906942607043474e-5 * t34038 - 0.3077456993052877797e-8 * t34043 - 0.15387284965264388985e-8 * t34046 + 0.99443481748595550042e-7 * t34048 - 0.10316808205282028424e-4 * t34050 + 0.1600868508130162607e-6 * t34052 + 0.14302847739140993952e-5 * t34054 + 0.70341874126922921073e-8 * t34056 + 0.23286599093046454432e-9 * t34060 + 0.24760339692676868218e-5 * t34062;
    (t34058, t34064)
}
