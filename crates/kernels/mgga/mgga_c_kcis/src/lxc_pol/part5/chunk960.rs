//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 960/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk960<F: Float>(t9725: F, t2937: F, t926: F, t270: F, t3030: F, t9728: F, t999: F, t292: F, t737: F, t285: F, t1071: F, t240: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9775 = F::new(0.28842592592592592592e-1) * t9725;
    let t9790 = F::new(0.55403703703703703703e-1) * t9725;
    let t9804 = t926 * t2937;
    let t9825 = F::new(1.0) / t3030 / t270;
    let t9851 = F::new(0.93932222222222222223e0) * t9725;
    let t9852 = F::new(0.36793333333333333333e0) * t9728;
    let t9873 = t999 * t999;
    let t9874 = F::new(1.0) / t9873;
    let t9881 = t737 * t292;
    let t9883 = F::new(5.0) / F::new(1296.0) * t285 * t9881;
    let t9896 = F::new(1.0) / t240 / t1071;
    (t9775, t9790, t9804, t9825, t9851, t9852, t9874, t9883, t9896)
}
