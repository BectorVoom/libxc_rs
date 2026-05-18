//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1099/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1099<F: Float>(t37637: F, t39469: F, t24906: F, t37943: F, t37945: F, t24916: F, t37949: F, t37616: F, t37630: F, t37634: F, t37639: F, t10810: F, t574: F, t8066: F) -> (F, F, F, F, F, F, F, F) {
    let t39470 = t37637 * t39469;
    let t39482 = t37943 * t37945 * t24906;
    let t39485 = t37949 * t37945 * t24916;
    let t39487 = F::new(0.84755945902752848174e0) * t37616;
    let t39492 = F::new(0.11902492299418487743e0) * t37630;
    let t39493 = F::new(0.35707476898255463229e0) * t37634;
    let t39494 = F::new(0.28914548798370980346e-3) * t37639;
    let t39499 = t574 * t10810 * t8066;
    (t39470, t39482, t39485, t39487, t39492, t39493, t39494, t39499)
}
