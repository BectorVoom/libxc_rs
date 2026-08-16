//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1166/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1166<F: Float>(t33550: F, t18610: F, t18619: F, t18624: F, t18626: F, t18629: F, t18645: F, t18655: F, t18658: F, t18667: F, t18709: F, t18914: F) -> (F, F) {
    let t48493 = F::cast_from(120.0_f64) * t33550;
    let t48494 = -t18610 - t18619 - t18624 + t48493 - t18626 - t18629 - t18645 + t18655 + t18658 - t18667 + t18709 + t18914;
    (t48493, t48494)
}
