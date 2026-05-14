//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 975/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk975<F: Float>(t33288: F, t35834: F, t7638: F, t35829: F, t33811: F, t35820: F, t143177: F, t143180: F, t152899: F, t152902: F, t152905: F, t152907: F, t152913: F, t152917: F, t152920: F, t152924: F, t152927: F, t152931: F, t152934: F) -> (F, F, F, F) {
    let t152937 = t7638 * t33288 * t35834;
    let t152940 = t7638 * t33288 * t35829;
    let t152943 = t33811 * t33288 * t35820;
    let t152945 = -8.0 / 9.0 * t152899 + 2.0 / 3.0 * t152902 - 2.0 / 9.0 * t152905 - t152907 / 54.0 + t143177 / 18.0 + t143180 / 3.0 - 2.0 * t152913 + t152917 / 18.0 + t152920 / 18.0 + 2.0 / 3.0 * t152924 - t152927 / 9.0 + t152931 / 9.0 - t152934 / 36.0 + 4.0 / 9.0 * t152937 - 8.0 / 9.0 * t152940 - t152943 / 3.0;
    (t152937, t152940, t152943, t152945)
}
