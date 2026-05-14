//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 490/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk490<F: Float>(t2059: F, t3532: F, t1390: F, t5: F, t969: F, t1173: F, t2188: F, t2083: F, t3598: F, t1171: F, t2079: F, t3651: F, t2089: F, t827: F, t22: F, t3118: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5670 = t3532 * t2059;
    let t5675 = t1390 * t2059;
    let t5680 = t5 * t969;
    let t5687 = t1173 * t2188;
    let t5690 = t3598 * t2083;
    let t5715 = t2079 * t1171;
    let t5730 = t3651 * t2083;
    let t5736 = t827 * t2089;
    let t5744 = t22 * t3118;
    (t5670, t5675, t5680, t5687, t5690, t5715, t5730, t5736, t5744)
}
