//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 683/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk683<F: Float>(t1564: F, t15951: F, t446: F, t15950: F, t2983: F, t7793: F, t3266: F, t925: F, t7824: F, t10993: F, t11022: F, t11024: F, t11026: F, t11070: F, t11404: F, t11417: F, t11659: F, t11781: F, t15934: F, t15938: F, t15942: F, t15945: F, t15948: F, t7775: F, t8190: F, t8192: F) -> (F, F, F, F, F, F) {
    let t15952 = t1564 * t15951;
    let t15953 = t446 * t15952;
    let t15955 = t2983 * t15950;
    let t15956 = t7793 * t15955;
    let t15957 = t446 * t15956;
    let t15959 = t925 * t3266;
    let t15960 = t7824 * t15959;
    let t15961 = t446 * t15960;
    let t15966 = t15934 / 9.0 + 2.0 / 9.0 * t15938 + t15942 / 27.0 + 2.0 / 9.0 * t15945 - 5.0 / 81.0 * t15948 - t10993 - t11022 - t11024 + t11026 - 2.0 / 9.0 * t15953 + 2.0 / 27.0 * t15957 - 2.0 / 9.0 * t15961 - 2.0 / 81.0 * t7775 - 2.0 / 27.0 * t8192 - t11659 + t11070 - t11781 - t8190 + 2.0 / 27.0 * t11404 - t11417;
    (t15953, t15955, t15957, t15959, t15961, t15966)
}
