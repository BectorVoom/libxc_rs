//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 578/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk578<F: Float>(t3130: F, t882: F, t2372: F, t901: F, t2366: F, t874: F, t2365: F, t1429: F, t123: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t3132 = F::cast_from(0.23712505529730124666e-2_f64) * t882 * t3130;
    let t3157 = F::cast_from(0.29792074959875355558e-1_f64) * t2372 * t901;
    let t3162 = t2366 * t874;
    let t3163 = t2365 * t3162;
    let t3165 = F::cast_from(0.29792074959875355558e-1_f64) * t1429 * t3163;
    let t3176 = t874 * t123;
    let t3177 = t3176 * t883;
    (t3132, t3157, t3162, t3163, t3165, t3176, t3177)
}
