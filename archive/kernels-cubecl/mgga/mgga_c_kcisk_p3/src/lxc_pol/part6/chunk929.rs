//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 929/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk929<F: Float>(t29503: F, t735: F, t1935: F, t2560: F, t9055: F, t742: F, t651: F, t79: F, t747: F, t741: F, t28256: F, t5290: F) -> (F, F, F, F) {
    let t29504 = t735 * t29503;
    let t29505 = t1935 * t29504;
    let t29507 = t2560 * t9055;
    let t29509 = t742 * t742;
    let t29512 = F::cast_from(1.0_f64) / t651 / t29509 * t79;
    let t29513 = t29512 * t747;
    let t29514 = t741 * t29513;
    let t29516 = t5290 * t28256;
    (t29505, t29507, t29514, t29516)
}
