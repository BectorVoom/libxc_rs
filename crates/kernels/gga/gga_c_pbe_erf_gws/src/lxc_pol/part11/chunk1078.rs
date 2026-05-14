//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1078/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1078<F: Float>(t35128: F, t18626: F, t18629: F, t18645: F, t18655: F, t18658: F, t18667: F, t18709: F, t18838: F, t18914: F, t19517: F, t48493: F, t48495: F, t48496: F, t22731: F, t22735: F) -> (F, F, F, F) {
    let t49423 = 0.37963457796989083263e1 * t35128;
    let t49424 = t48493 - t18626 - t18629 - t18645 + t18655 + t18658 - t18667 + t18709 + t18914 - t48495 - t48496 - t18838 - t19517 - t49423;
    let t49425 = 0.18960024086108224108e1 * t22731;
    let t49426 = 0.73024584604562962965e1 * t22735;
    (t49423, t49424, t49425, t49426)
}
