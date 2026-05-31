//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2592/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2592<F: Float>(t1774: F, t487: F, t45928: F, t45934: F, t45938: F, t45945: F, t45949: F, t2246: F, t4171: F, t10308: F, t1466: F, t10355: F, t44: F) -> (F, F, F, F, F, F, F, F, F) {
    let t60037 = t487 * t1774;
    let t60214 = F::cast_from(96.0_f64) * t45928;
    let t60215 = F::cast_from(192.0_f64) * t45934;
    let t60216 = F::cast_from(960.0_f64) * t45938;
    let t60217 = F::cast_from(1440.0_f64) * t45945;
    let t60218 = F::cast_from(4032.0_f64) * t45949;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60308 = t44 * t10355;
    (t60037, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60308)
}
