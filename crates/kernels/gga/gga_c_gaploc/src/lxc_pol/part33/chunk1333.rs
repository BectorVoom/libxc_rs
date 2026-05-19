//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1333/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1333<F: Float>(t1411: F, t3395: F, t587: F, t2365: F, t2366: F, t4379: F, t7892: F, t10241: F, t9448: F, t15482: F, t20560: F, t9439: F) -> (F, F, F, F) {
    let t34796 = t587 * t1411 * t3395;
    let t34797 = F::cast_from(0.59644551483876721719e0_f64) * t34796;
    let t34800 = t4379 * t2365 * t2366 * t7892;
    let t34801 = F::cast_from(0.89376224879626066674e-1_f64) * t34800;
    let t34814 = t9448 * t10241;
    let t34817 = F::cast_from(0.5680433474654925878e0_f64) * t20560 * t15482 * t34814;
    let t34818 = t9439 * t10241;
    (t34797, t34801, t34817, t34818)
}
