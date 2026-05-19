//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1143/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1143<F: Float>(t39251: F, t39255: F, t39260: F, t40312: F, t40341: F, t40345: F, t40425: F, t40428: F, t40434: F, t40456: F, t40460: F, t40518: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42164 = F::cast_from(0.1440846329149835838e-2_f64) * t39251;
    let t42165 = F::cast_from(0.1440846329149835838e-2_f64) * t39255;
    let t42170 = F::cast_from(0.162600798888400151e-2_f64) * t39260;
    let t42187 = F::cast_from(0.1440846329149835838e-2_f64) * t40312;
    let t42196 = F::cast_from(0.1440846329149835838e-2_f64) * t40341;
    let t42197 = F::cast_from(0.20496175532535769482e-3_f64) * t40345;
    let t42208 = F::cast_from(0.1440846329149835838e-2_f64) * t40425;
    let t42209 = F::cast_from(0.20496175532535769482e-3_f64) * t40428;
    let t42210 = F::cast_from(0.3842256877732895568e-2_f64) * t40434;
    let t42215 = F::cast_from(0.60975299583150056624e-3_f64) * t40456;
    let t42216 = F::cast_from(0.86737941314158990616e-4_f64) * t40460;
    let t42229 = F::cast_from(0.60975299583150056624e-3_f64) * t40518;
    (t42164, t42165, t42170, t42187, t42196, t42197, t42208, t42209, t42210, t42215, t42216, t42229)
}
