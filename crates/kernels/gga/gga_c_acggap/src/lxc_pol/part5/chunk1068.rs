//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1068/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1068<F: Float>(t4147: F, t4180: F, t16986: F, t4200: F, t12309: F, t4242: F, t12313: F, t1629: F, t16560: F, t4159: F, t12726: F, t1642: F) -> (F, F, F, F, F, F) {
    let t18925 = t4180 * t4147;
    let t18930 = t16986 * t4200;
    let t18935 = t12309 * t4242;
    let t18938 = t12313 * t1629 * t16560;
    let t18941 = t4180 * t4159;
    let t18951 = t12726 * t1642;
    (t18925, t18930, t18935, t18938, t18941, t18951)
}
