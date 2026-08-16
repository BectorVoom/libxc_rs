//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk693;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta156<F: Float>(t4306: F, t190: F, t4186: F, t706: F, t1531: F, t705: F, t707: F, t2498: F, t2518: F, t2522: F, t2526: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t4300: F, t4301: F, t4304: F) -> (F, F, F, F, F, F) {
        let (t4307, t4308, t4310, t4311) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk693::<F>(t4306, t190, t4186, t706, t1531, t705);
        let (t4313, t4314) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk694::<F>(t4311, t707, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304, t4307, t4310);
    (t4307, t4308, t4310, t4311, t4313, t4314)
}
