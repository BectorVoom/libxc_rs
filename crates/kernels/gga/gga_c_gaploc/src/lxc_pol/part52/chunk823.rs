//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 823/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk823<F: Float>(t13265: F, t2312: F, t1063: F, t11264: F, t6755: F, t2268: F, t35045: F, t7937: F, t42827: F, t11232: F, t894: F, t2440: F, t3531: F) -> (F, F, F, F, F, F) {
    let t44543 = t2312 * t13265;
    let t44544 = F::cast_from(0.35568758294595186999e-2_f64) * t44543;
    let t44549 = F::cast_from(0.34146007962811379518e0_f64) * t1063 * t11264 * t6755;
    let t44552 = F::cast_from(0.68292015925622759036e0_f64) * t2268 * t7937 * t35045;
    let t44553 = F::cast_from(0.47425011059460249332e-2_f64) * t42827;
    let t44556 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t894 * t11232;
    let t44559 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t2440 * t3531;
    (t44544, t44549, t44552, t44553, t44556, t44559)
}
