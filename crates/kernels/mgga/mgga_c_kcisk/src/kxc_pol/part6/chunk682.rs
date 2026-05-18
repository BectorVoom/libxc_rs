//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 682/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk682<F: Float>(t10487: F, t708: F, t10671: F, t677: F, t10568: F, t5101: F, t707: F, t1797: F, t180: F, t479: F, t574: F, t682: F) -> (F, F, F, F, F, F) {
    let t11328 = t708 * t10487;
    let t11352 = t10671 * t677;
    let t11371 = F::new(0.12841111111111111111e-1) * t10568;
    let t11393 = t707 * t5101;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    (t11328, t11352, t11371, t11393, t11400, t11401)
}
