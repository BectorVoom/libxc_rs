//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1206/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1206<F: Float>(t1105: F, t13087: F, t13720: F, t18577: F, t18580: F, t18587: F, t18594: F, t18599: F, t18604: F, t18607: F, t18610: F, t18619: F, t18624: F, t18626: F, t18629: F, t18645: F, t2429: F, t48489: F, t48493: F, t804: F) -> F {
    let t48957 = F::new(24.0) * t1105 * t13087 * t2429 + F::new(24.0) * t1105 * t13720 * t804 + t18577 + t18580 + t18587 + t18594 + t18599 - t18604 - t18607 - t18610 - t18619 - t18624 - t18626 - t18629 - t18645 - t48489 + t48493;
    t48957
}
