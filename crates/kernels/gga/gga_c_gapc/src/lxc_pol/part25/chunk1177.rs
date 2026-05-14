//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1177/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1177<F: Float>(t34367: F, t34370: F, t34378: F, t34380: F, t34383: F, t34386: F, t34388: F, t34390: F, t34394: F, t34397: F, t34400: F, t34403: F, t34406: F, t34421: F, t34424: F, t34426: F) -> (F, F, F, F, F, F) {
    let t36914 = 0.2318836277704281739e-4 * t34367;
    let t36915 = 0.4637672555408563478e-4 * t34370;
    let t36930 = 0.34752370105806885418e-3 * t34378 + 0.45020247456651744776e-7 * t34380 + 0.45020247456651744776e-6 * t34383 + 0.45020247456651744776e-7 * t34386 + 0.45020247456651744776e-6 * t34388 - 0.2318836277704281739e-4 * t34390 - 0.69504740211613770836e-3 * t34394 - 0.41223756048076119805e-5 * t34397 - 0.43440462632258606772e-4 * t34400 - 0.43440462632258606772e-4 * t34403 - 0.21720231316129303386e-4 * t34406;
    let t36934 = 0.48917046440972222224e-4 * t34421;
    let t36935 = 0.25002399603899953676e-2 * t34424;
    let t36936 = 0.3243554543208642639e-2 * t34426;
    (t36914, t36915, t36930, t36934, t36935, t36936)
}
