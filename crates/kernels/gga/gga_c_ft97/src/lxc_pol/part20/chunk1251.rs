//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1251/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1251<F: Float>(t193: F, t24964: F, t4129: F, t89: F, t14889: F, t6222: F, t113190: F, t14690: F, t99529: F, t10248: F, t113197: F, t446: F, t1882: F, t28517: F, t28521: F, t99584: F, t99607: F, t99812: F, t99822: F, t99823: F, t99826: F) -> (F, F, F, F, F, F, F) {
    let t113618 = t89 * t193 * t24964 * t4129;
    let t113622 = t89 * t193 * t6222 * t14889;
    let t113625 = t113190 * t99529 * t14690;
    let t113629 = t446 * t10248 * t113197;
    let t113631 = t1882 * t28517;
    let t113632 = 4.0 / 9.0 * t113631;
    let t113633 = t1882 * t28521;
    let t113634 = 4.0 / 9.0 * t113633;
    let t113635 = 4.0 * t113618 + 2.0 * t113622 + t99812 + t99584 - t99822 + 4.0 / 3.0 * t113625 + t99823 + 16.0 / 9.0 * t99607 - t99826 - 4.0 / 3.0 * t113629 + t113632 + t113634;
    (t113618, t113622, t113625, t113629, t113631, t113633, t113635)
}
