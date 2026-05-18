//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1072/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1072<F: Float>(t2247: F, t4188: F, t4248: F, t491: F, t1528: F, t4254: F) -> (F, F, F, F) {
    let t27494 = t2247 * t4188;
    let t27514 = t4248 * t491;
    let t27517 = t1528 * t491;
    let t27520 = t4254 * t491;
    (t27494, t27514, t27517, t27520)
}
