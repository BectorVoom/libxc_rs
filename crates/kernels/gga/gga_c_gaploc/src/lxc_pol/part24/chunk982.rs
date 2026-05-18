//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 982/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk982<F: Float>(t9664: F, t9666: F, t9669: F, t9672: F, t9674: F, t9676: F, t471: F, t3427: F, t64: F, t2919: F, t871: F) -> (F, F) {
    let t10657 = -F::new(21.0) / F::new(256.0) * t9664 + F::new(147.0) / F::new(8192.0) * t9666 - F::new(63.0) / F::new(524288.0) * t9669 + F::new(21.0) / F::new(524288.0) * t9672 - F::new(49.0) / F::new(8192.0) * t9674 + F::new(7.0) / F::new(256.0) * t9676;
    let t10658 = t10657 * t471;
    let t10660 = F::new(4.0) / F::new(3.0) * t3427 * t64;
    let t10661 = t2919 * t871;
    let t10663 = F::new(7.0) / F::new(256.0) * t9664;
    let t10664 = F::new(21.0) / F::new(8192.0) * t9666;
    let t10665 = F::new(7.0) / F::new(8192.0) * t9674;
    let t10666 = F::new(7.0) / F::new(768.0) * t9676;
    let t10667 = t10658 - t10660 + t10661 / F::new(2.0) - t10663 + t10664 - t10665 + t10666;
    (t10657, t10667)
}
