//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 615/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk615<F: Float>(t3682: F, t4026: F, t4399: F, t1851: F, t971: F, t1882: F, t3010: F, t2989: F, t2994: F, t2985: F, t7775: F, t8192: F, t7773: F, t89: F, t921: F, t3104: F, t375: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10948 = 2.0 * t3682;
    let t10949 = 2.0 * t4026;
    let t10950 = 2.0 * t4399;
    let t10969 = t971 * t1851;
    let t10992 = t1882 * t3010;
    let t10993 = t10992 / 27.0;
    let t11021 = t1882 * t2989;
    let t11022 = t11021 / 27.0;
    let t11023 = t1882 * t2994;
    let t11024 = 2.0 / 27.0 * t11023;
    let t11025 = t1882 * t2985;
    let t11026 = 2.0 / 81.0 * t11025;
    let t11027 = 4.0 / 81.0 * t7775;
    let t11031 = 4.0 / 27.0 * t8192;
    let t11043 = t89 * t7773 * t921;
    let t11069 = t89 * t375 * t3104;
    (t10948, t10949, t10950, t10969, t10992, t10993, t11021, t11022, t11023, t11024, t11025, t11026, t11027, t11031, t11043, t11069)
}
