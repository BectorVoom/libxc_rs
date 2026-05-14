//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 679/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk679<F: Float>(t15885: F, t370: F, t27: F, t89: F, t375: F, t4496: F, t4437: F, t15746: F, t1866: F, t3281: F, t1882: F, t4423: F, t358: F, t4495: F, t363: F, t1564: F) -> (F, F, F, F, F, F, F) {
    let t15886 = t370 * t15885;
    let t15888 = t89 * t27 * t15886;
    let t15891 = t89 * t375 * t4496;
    let t15894 = t89 * t375 * t4437;
    let t15896 = t1866 * t15746;
    let t15897 = t3281 * t15896;
    let t15899 = t1882 * t4423;
    let t15901 = t4495 * t358;
    let t15902 = t15901 * t363;
    let t15903 = t1564 * t15902;
    (t15888, t15891, t15894, t15897, t15899, t15902, t15903)
}
