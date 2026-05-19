//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1002/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1002<F: Float>(t1445: F, t43001: F, t833: F, t3234: F, t813: F, t8528: F, t2949: F, t9688: F, t13130: F, t2194: F, t32179: F, t935: F) -> (F, F, F, F, F) {
    let t43950 = t833 * t1445 * t43001;
    let t43955 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t8528 * t3234;
    let t43959 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t2949 * t9688;
    let t43961 = F::cast_from(0.46011511144704899612e1_f64) * t2194 * t13130;
    let t43964 = t813 * t1445 * t32179 * t935;
    (t43950, t43955, t43959, t43961, t43964)
}
