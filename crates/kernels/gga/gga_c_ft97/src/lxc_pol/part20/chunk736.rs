//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 736/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk736<F: Float>(t280: F, t39: F, t2035: F, t1208: F, t820: F, t291: F, t4092: F, t817: F, t1200: F, t283: F, t811: F, t800: F, t285: F, t1240: F, t2766: F, t11: F, t5585: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19038 = t280 * t39;
    let t19039 = t19038 * t2035;
    let t19072 = t1208 * t820;
    let t19100 = t291 * t39;
    let t19101 = t4092 * t19100;
    let t19106 = t817 * t39;
    let t19107 = t1200 * t19106;
    let t19116 = t811 * t283;
    let t19132 = t800 * t19100;
    let t19135 = t285 * t19106;
    let t19500 = t2766 * t1240;
    let t22511 = t5585 * t11;
    (t19038, t19039, t19072, t19101, t19106, t19107, t19116, t19132, t19135, t19500, t22511)
}
