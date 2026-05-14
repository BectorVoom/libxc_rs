//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 498/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk498<F: Float>(t1501: F, t1517: F, t4176: F, t4183: F, t4186: F, t4190: F, t4194: F, t4198: F, t4201: F, t4206: F, t4212: F, t4216: F, t4218: F, t3507: F, t492: F, t1506: F) -> (F, F, F, F) {
    let t4220 = t1501 * t1517;
    let t4222 = t4176 / 24.0 - 19.0 / 144.0 * t4183 + t4186 / 18.0 + t4190 / 256.0 - t4194 / 192.0 - t4198 / 16.0 + t4201 / 3.0 - t4206 / 12.0 + t4212 / 8.0 - t4216 / 8.0 + t4218 / 24.0 - t4220 / 96.0;
    let t4223 = t3507 * t492;
    let t4224 = t4223 * t1506;
    (t4220, t4222, t4223, t4224)
}
