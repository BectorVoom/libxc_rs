//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 853/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk853<F: Float>(t7222: F, t8232: F, t1882: F, t32520: F, t32599: F, t8392: F, t32532: F, t32529: F, t32536: F, t32483: F, t32617: F, t32573: F, t32524: F, t7231: F, t32559: F, t32564: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t138119 = 8.0 / 27.0 * t8232 * t7222;
    let t138126 = t1882 * t32520;
    let t138143 = t8392 * t32599;
    let t138154 = t1882 * t32532;
    let t138156 = t1882 * t32529;
    let t138158 = t1882 * t32536;
    let t138168 = t1882 * t32483;
    let t138176 = t1882 * t32617;
    let t138178 = t1882 * t32573;
    let t138184 = t1882 * t32524;
    let t138208 = 8.0 / 27.0 * t8232 * t7231;
    let t138221 = t1882 * t32559;
    let t138223 = t1882 * t32564;
    (t138119, t138126, t138143, t138154, t138156, t138158, t138168, t138176, t138178, t138184, t138208, t138221, t138223)
}
