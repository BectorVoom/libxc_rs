//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 860/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk860<F: Float>(t12952: F, t3661: F, t26: F, t1186: F, t12957: F, t3665: F, t827: F, t303: F, t1175: F, t3559: F, t1394: F, t298: F, t301: F) -> (F, F, F, F, F, F) {
    let t12961 = t3661 * t12952;
    let t12962 = t26 * t12961;
    let t12964 = t1186 * t12957;
    let t12965 = t26 * t12964;
    let t12967 = t827 * t3665;
    let t12969 = F::cast_from(1.0_f64)/pow_3_2::<F>(t303);
    let t12970 = t3559 * t1175;
    let t12971 = t12969 * t12970;
    let t12974 = t298 * t1394 * t301;
    (t12962, t12965, t12967, t12970, t12971, t12974)
}
