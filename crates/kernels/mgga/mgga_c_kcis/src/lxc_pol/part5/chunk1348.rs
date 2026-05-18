//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1348/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1348<F: Float>(t22265: F, t5661: F, t11862: F, t6905: F, t167: F, t2011: F, t4171: F, t4170: F, t16771: F, t1307: F, t7392: F, t12241: F) -> (F, F, F, F) {
    let t22266 = t5661 * t22265;
    let t22268 = t11862 * t6905;
    let t22270 = t167 * t2011;
    let t22271 = t4171 * t22270;
    let t22272 = t4170 * t22271;
    let t22273 = t16771 * t22272;
    let t22275 = t7392 * t1307;
    let t22276 = t12241 * t22275;
    (t22266, t22268, t22273, t22276)
}
