//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 732/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk732<F: Float>(t7584: F, t875: F, t2862: F, t871: F, t319: F, t33835: F, t1882: F, t7635: F, t7626: F, t7622: F, t1901: F, t34156: F, t34158: F, t34160: F, t34164: F, t34169: F, t34174: F, t34178: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t34181 = t7584 * t875;
    let t34183 = t2862 * t871 * t34181;
    let t34187 = t2862 * t319 * t33835;
    let t34191 = t1882 * t7635 / 9.0;
    let t34193 = 2.0 / 9.0 * t1882 * t7626;
    let t34195 = 2.0 / 9.0 * t1882 * t7622;
    let t34196 = t34156 - t34158 - 2.0 / 9.0 * t1901 * t34160 + 2.0 / 3.0 * t446 * t34164 - 2.0 / 3.0 * t446 * t34169 - 2.0 * t446 * t34174 - 2.0 * t446 * t34178 - 2.0 / 3.0 * t446 * t34183 + 4.0 / 3.0 * t446 * t34187 + t34191 + t34193 - t34195;
    (t34181, t34183, t34187, t34191, t34193, t34195, t34196)
}
