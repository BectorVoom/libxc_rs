//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 970/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk970<F: Float>(t18594: F, t18596: F, t18599: F, t18601: F, t18604: F, t18607: F, t18610: F, t18612: F, t18614: F, t18619: F, t18624: F, t18626: F, t18629: F, t18631: F, t18634: F, t18636: F, t4379: F, t804: F, t946: F) -> (F,) {
    let t19498 = 12.0 * t4379 * t804 * t946 + t18594 + t18596 + t18599 - t18601 - t18604 - t18607 - t18610 + t18612 - t18614 - t18619 - t18624 + t18626 - t18629 - t18631 - t18634 - t18636;
    (t19498,)
}
