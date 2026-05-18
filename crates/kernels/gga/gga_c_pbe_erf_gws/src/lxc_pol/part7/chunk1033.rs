//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1033/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1033<F: Float>(t1399: F, t4782: F, t1218: F, t4793: F, t18624: F, t18626: F, t18629: F, t18631: F, t18634: F, t18636: F, t18645: F, t18647: F, t18655: F, t18658: F) -> (F, F, F) {
    let t18659 = t1399 * t4782;
    let t18660 = F::new(0.41015588084031179722e4) * t18659;
    let t18661 = t4793 * t1218;
    let t18662 = F::new(0.70178680769462448852e1) * t18661;
    let t18663 = -t18624 + t18626 - t18629 - t18631 - t18634 - t18636 - t18645 - t18647 + t18655 + t18658 - t18660 + t18662;
    (t18660, t18662, t18663)
}
