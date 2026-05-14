//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1072/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1072<F: Float>(t14728: F, t800: F, t4092: F, t10364: F, t6: F, t1200: F, t285: F, t230: F, t3750: F, t10696: F, t1240: F, t2842: F, t4239: F, t2770: F, t4246: F, t309: F, t798: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54840 = t800 * t14728;
    let t54924 = t4092 * t14728;
    let t54927 = t10364 * t6;
    let t54928 = t1200 * t54927;
    let t55011 = t285 * t54927;
    let t55105 = t230 * t3750;
    let t55768 = t1240 * t10696;
    let t55797 = t4239 * t2842;
    let t56098 = t2770 * t4246;
    let t56110 = t798 * t309;
    (t54840, t54924, t54928, t55011, t55105, t55768, t55797, t56098, t56110)
}
