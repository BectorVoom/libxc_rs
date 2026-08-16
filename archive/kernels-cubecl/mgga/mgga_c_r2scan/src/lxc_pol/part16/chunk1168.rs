//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1168/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1168<F: Float>(t11554: F, t2850: F, t3275: F, t3276: F, t11555: F, t11629: F, t1100: F, t42929: F, t42931: F, t42933: F, t42937: F, t42939: F, t42943: F, t42947: F, t42949: F, t42951: F, t42953: F, t42958: F, t42962: F, t42965: F, t9832: F) -> (F, F, F) {
    let t42966 = t11554 * t2850;
    let t42969 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3276 * t42966;
    let t42972 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t11629 * t11555;
    let t42973 = t1100 * t9832 - t42929 - t42931 + t42933 + t42937 - t42939 + t42943 + t42947 - t42949 - t42951 - t42953 - t42958 - t42962 + t42965 + t42969 + t42972;
    (t42969, t42972, t42973)
}
