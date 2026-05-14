//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1024/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1024<F: Float>(t33550: F, t18610: F, t18619: F, t18624: F, t18626: F, t18629: F, t18645: F, t18655: F, t18658: F, t18667: F, t18709: F, t18914: F, t22634: F, t22636: F, t33572: F, t22641: F) -> (F, F, F, F, F, F) {
    let t48493 = 120.0 * t33550;
    let t48494 = -t18610 - t18619 - t18624 + t48493 - t18626 - t18629 - t18645 + t18655 + t18658 - t18667 + t18709 + t18914;
    let t48495 = 576.0 * t22634;
    let t48496 = 960.0 * t22636;
    let t48497 = 0.1038945353962551798e3 * t33572;
    let t48498 = 0.41015588084031179722e4 * t22641;
    (t48493, t48494, t48495, t48496, t48497, t48498)
}
