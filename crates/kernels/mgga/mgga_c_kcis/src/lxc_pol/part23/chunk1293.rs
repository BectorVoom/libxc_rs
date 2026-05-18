//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1293/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1293<F: Float>(t5440: F, t99198: F, t99199: F, t1307: F, t28814: F, t95024: F, t3978: F, t7969: F, t5426: F, t1370: F, t27636: F, t27606: F, t6140: F) -> (F, F, F, F, F) {
    let t99201 = t99198 * t5440 * t99199;
    let t99205 = t95024 * t28814 * t1307;
    let t99208 = t3978 * t7969;
    let t99210 = t99208 * t5426 * t99199;
    let t99213 = t1370 * t27636;
    let t99219 = t27606 * t6140;
    (t99201, t99205, t99210, t99213, t99219)
}
