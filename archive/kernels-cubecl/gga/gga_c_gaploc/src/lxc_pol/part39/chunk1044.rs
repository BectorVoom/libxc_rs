//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1044/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1044<F: Float>(t13033: F, t4614: F, t5748: F, t1445: F, t2087: F, t33118: F, t935: F, t1991: F, t42944: F, t590: F, t739: F, t1890: F, t1966: F) -> (F, F, F, F) {
    let t43864 = F::cast_from(0.36809208915763919689e2_f64) * t5748 * t4614 * t13033;
    let t43870 = t2087 * t1445 * t33118 * t935;
    let t43875 = F::cast_from(0.20449560508757733161e1_f64) * t1991 * t739 * t42944 * t590;
    let t43879 = F::cast_from(0.97135412416599232513e1_f64) * t1966 * t1890 * t42944 * t590;
    (t43864, t43870, t43875, t43879)
}
