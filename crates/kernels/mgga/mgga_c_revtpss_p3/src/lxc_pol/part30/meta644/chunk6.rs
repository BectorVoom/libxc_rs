//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2267/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2267<F: Float>(t29468: F, t575: F, t1464: F, t8240: F, t1921: F, t7690: F, t2167: F, t5808: F, t2172: F, t5789: F, t1913: F, t7700: F) -> (F, F, F, F, F, F) {
    let t105792 = F::cast_from(2.0_f64) * t29468 * t575;
    let t105794 = F::cast_from(2.0_f64) * t8240 * t1464;
    let t105796 = F::cast_from(2.0_f64) * t7690 * t1921;
    let t105798 = F::cast_from(2.0_f64) * t2167 * t5808;
    let t105800 = F::cast_from(2.0_f64) * t5789 * t2172;
    let t105802 = F::cast_from(2.0_f64) * t1913 * t7700;
    (t105792, t105794, t105796, t105798, t105800, t105802)
}
