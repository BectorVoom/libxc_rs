//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1036/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1036<F: Float>(t11621: F, t3275: F, t40687: F, t11554: F, t2850: F, t3276: F, t11555: F, t11629: F, t1100: F, t42929: F, t42931: F, t42933: F, t42937: F, t42939: F, t42943: F, t42947: F, t42949: F, t42951: F, t42953: F, t42958: F, t42962: F, t9832: F) -> (F, F, F, F) {
    let t42965 = 45.0 / 32.0 * t3275 * t40687 * t11621;
    let t42966 = t11554 * t2850;
    let t42969 = 5.0 / 8.0 * t3275 * t3276 * t42966;
    let t42972 = 5.0 / 8.0 * t3275 * t11629 * t11555;
    let t42973 = t1100 * t9832 - t42929 - t42931 + t42933 + t42937 - t42939 + t42943 + t42947 - t42949 - t42951 - t42953 - t42958 - t42962 + t42965 + t42969 + t42972;
    (t42965, t42969, t42972, t42973)
}
