//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 580/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk580<F: Float>(t2197: F, t3492: F, t10713: F, t1445: F, t833: F, t10717: F, t1022: F, t5241: F, t2679: F, t9805: F, t1029: F, t9796: F) -> (F, F, F, F, F, F, F) {
    let t11043 = F::new(0.11502877786176224903e2) * t2197 * t3492;
    let t11044 = t1445 * t10713;
    let t11046 = F::new(0.11502877786176224903e2) * t833 * t11044;
    let t11047 = t1445 * t10717;
    let t11049 = F::new(0.11502877786176224903e2) * t833 * t11047;
    let t11053 = t5241 * t1022;
    let t11054 = t11053 * t2679;
    let t11055 = t9805 * t11054;
    let t11056 = F::new(0.57514388930881124514e0) * t11055;
    let t11057 = t1029 * t2679;
    let t11058 = t9796 * t11057;
    (t11043, t11046, t11049, t11053, t11055, t11056, t11058)
}
