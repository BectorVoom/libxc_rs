//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 877/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk877<F: Float>(t2615: F, t326: F, t45316: F, t43832: F, t1890: F, t1966: F, t44707: F, t590: F, t10024: F, t11823: F, t43881: F, t5241: F, t5640: F) -> (F, F, F, F, F, F) {
    let t45658 = F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t45316;
    let t45663 = F::cast_from(0.23005755572352449806e1_f64) * t43832;
    let t45667 = F::cast_from(0.97135412416599232513e1_f64) * t1966 * t1890 * t44707 * t590;
    let t45678 = t11823 * t10024;
    let t45680 = F::cast_from(0.15337170381568299871e1_f64) * t43881;
    let t45684 = F::cast_from(0.13803453343411469884e2_f64) * t5640 * t5241 * t44707 * t590;
    (t45658, t45663, t45667, t45678, t45680, t45684)
}
