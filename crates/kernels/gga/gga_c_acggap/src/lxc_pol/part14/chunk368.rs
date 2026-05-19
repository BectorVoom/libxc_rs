//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 368/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk368<F: Float>(t1007: F, t1034: F, t1041: F, t1173: F, t1180: F, t165: F, t1745: F, t1750: F, t1755: F, t976: F, t979: F, t983: F, t989: F, t995: F) -> F {
    let t1758 = F::cast_from(0.42874018118069736972e-3_f64) * t165 * t1745 + t976 - t979 + t983 + t989 - t995 - t1007 + t1034 + t1041 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t1750 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1755;
    t1758
}
