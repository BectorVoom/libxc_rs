//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 921/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk921<F: Float>(t11765: F, t2714: F, t45305: F, t7572: F, t7573: F, t2615: F, t326: F, t45316: F, t43832: F, t1890: F, t1966: F, t44707: F, t590: F) -> (F, F, F, F, F) {
    let t45648 = F::cast_from(0.35750489951850426669e0_f64) * t2714 * t11765;
    let t45653 = F::cast_from(0.69017266717057349418e1_f64) * t7572 * t7573 * t45305;
    let t45658 = F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t45316;
    let t45663 = F::cast_from(0.23005755572352449806e1_f64) * t43832;
    let t45667 = F::cast_from(0.97135412416599232513e1_f64) * t1966 * t1890 * t44707 * t590;
    (t45648, t45653, t45658, t45663, t45667)
}
