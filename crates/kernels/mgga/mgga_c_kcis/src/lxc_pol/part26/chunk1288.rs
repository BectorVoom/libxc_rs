//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1288/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1288<F: Float>(t21947: F, t51692: F, t7923: F, t1394: F, t28356: F, t5637: F, t28499: F, t2109: F, t27596: F, t6176: F, t6188: F, t28727: F, t28741: F) -> (F, F, F, F, F) {
    let t102051 = t51692 * t7923 * t21947;
    let t102054 = t1394 * t28356 * t5637;
    let t102057 = t1394 * t28499 * t5637;
    let t102061 = t6176 * t27596 * t2109 * t6188;
    let t102064 = t28727 * t28741;
    (t102051, t102054, t102057, t102061, t102064)
}
