//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 683/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk683<F: Float>(t174: F, t236: F, t5398: F, t233: F, t1301: F, t1881: F, t1641: F, t4532: F, t447: F, t637: F, t446: F, t1640: F, t1885: F, t1300: F, t2132: F, t2002: F, t3734: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t5399 = t236 * t5398;
    let t5400 = t233 * t5399;
    let t5402 = t1881 * t1301;
    let t5404 = t1881 * t1641;
    let t5406 = piecewise3(t175, 0.0, -t4532);
    let t5407 = t447 * t5406;
    let t5408 = t5407 * t637;
    let t5409 = t446 * t5408;
    let t5411 = t1885 * t1640;
    let t5412 = t446 * t5411;
    let t5414 = t1300 * t2132;
    let t5415 = t446 * t5414;
    let t5417 = t3734 * t2002;
    (t5400, t5402, t5404, t5407, t5408, t5409, t5411, t5412, t5414, t5415, t5417)
}
