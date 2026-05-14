//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 730/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk730<F: Float>(t1011: F, t1656: F, t3115: F, t3234: F, t3241: F, t3245: F, t4887: F, t4892: F, t4896: F, t4899: F, t4902: F, t4907: F, t4912: F, t4916: F, t4920: F, t4925: F) -> (F,) {
    let t4928 = -t3241 * t1656 / 108.0 + t1011 * t4887 / 288.0 + 0.42874018118069736972e-3 * t4892 * t4896 - 0.21437009059034868486e-3 * t4899 * t4902 - 0.21437009059034868486e-3 * t3115 * t4907 - 0.21437009059034868486e-3 * t3115 * t4912 - t1011 * t4916 / 144.0 + t1011 * t4920 / 216.0 - 0.76220476654346199061e-3 * t3234 + t4925 / 864.0 + t3245 / 864.0;
    (t4928,)
}
