//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 769/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk769<F: Float>(t15944: F, t446: F, t15737: F, t8577: F, t363: F, t942: F, t2992: F, t1564: F, t2983: F, t7793: F, t3266: F, t925: F) -> (F, F, F, F, F, F, F) {
    let t15945 = t446 * t15944;
    let t15947 = t8577 * t15737;
    let t15948 = t446 * t15947;
    let t15950 = t942 * t363;
    let t15951 = t2992 * t15950;
    let t15952 = t1564 * t15951;
    let t15953 = t446 * t15952;
    let t15955 = t2983 * t15950;
    let t15956 = t7793 * t15955;
    let t15957 = t446 * t15956;
    let t15959 = t925 * t3266;
    (t15945, t15948, t15951, t15953, t15955, t15957, t15959)
}
