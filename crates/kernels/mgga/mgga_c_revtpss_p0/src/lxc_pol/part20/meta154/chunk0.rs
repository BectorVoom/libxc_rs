//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 838/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk838<F: Float>(t2782: F, t4089: F, t1419: F, t545: F, t869: F, t689: F, t136: F, t555: F, t2457: F, t3964: F, t4086: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t4090 = t2782 * t4089;
    let t4092 = t545 * t1419;
    let t4093 = t869 * t4092;
    let t4094 = t689 * t4093;
    let t4096 = t555 * t136;
    let t4099 = F::cast_from(0.11565819519348392139e-2_f64) * t3964 * t4096 * t2457;
    let t4100 = t4086 * t555;
    let t4101 = t786 * t4100;
    (t4090, t4092, t4093, t4094, t4096, t4099, t4100, t4101)
}
