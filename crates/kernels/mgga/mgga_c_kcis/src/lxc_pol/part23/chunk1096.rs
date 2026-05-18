//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1096/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1096<F: Float>(t2243: F, t5870: F, t303: F, t1458: F, t8175: F, t3964: F, t6140: F, t1385: F, t1650: F, t27356: F, t5709: F, t27453: F, t5654: F) -> (F, F, F, F, F, F, F, F) {
    let t28473 = t5870 * t2243;
    let t28474 = t303 * t28473;
    let t28476 = t1458 * t8175;
    let t28477 = t303 * t28476;
    let t28480 = t3964 * t6140;
    let t28483 = t1650 * t1385;
    let t28484 = t27356 * t28483;
    let t28485 = t5709 * t28484;
    let t28488 = t27453 * t5654;
    (t28473, t28474, t28476, t28477, t28480, t28484, t28485, t28488)
}
