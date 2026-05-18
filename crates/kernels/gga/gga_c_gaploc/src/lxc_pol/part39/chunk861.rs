//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 861/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk861<F: Float>(t2317: F, t6525: F, t9066: F, t1365: F, t30209: F, t9074: F, t9086: F, t9204: F, t29970: F, t4261: F, t29985: F, t30140: F) -> (F, F, F, F, F, F) {
    let t39808 = t6525 * t9066 * t2317;
    let t39811 = t6525 * t1365 * t30209;
    let t39849 = t9074 * t9204 * t9086;
    let t39866 = t6525 * t4261 * t29970;
    let t39869 = t9074 * t4261 * t29985;
    let t39893 = t9074 * t1365 * t30140;
    (t39808, t39811, t39849, t39866, t39869, t39893)
}
