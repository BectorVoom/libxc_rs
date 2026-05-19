//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1094/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1094<F: Float>(t11808: F, t16181: F, t9863: F, t667: F, t8709: F, t17891: F, t29070: F, t1736: F, t188: F, t1180: F, t11970: F, t1084: F, t327: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t33536 = t11808 * t9863 * t16181;
    let t33539 = t667 * t8709 * pi;
    let t33541 = t17891 * t33539 * t29070;
    let t33543 = t188 * t1736;
    let t33546 = t11970 * t1180;
    let t33547 = t1084 * t33543 * t327 * t33546;
    (t33536, t33539, t33541, t33543, t33546, t33547)
}
