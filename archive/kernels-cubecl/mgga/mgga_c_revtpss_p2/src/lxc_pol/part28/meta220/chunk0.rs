//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1042/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1042<F: Float>(t1012: F, t1014: F, t4579: F, t3252: F, t4574: F, t140: F, t1655: F, t1011: F, t1656: F, t3115: F, t3234: F, t3241: F, t3245: F, t4887: F, t4892: F, t4896: F, t4899: F, t4902: F, t4907: F, t4912: F) -> (F, F, F, F, F, F) {
    let t4915 = t1012 * t1014;
    let t4916 = t4915 * t4579;
    let t4919 = t1012 * t3252;
    let t4920 = t4919 * t4574;
    let t4924 = t140 * t1655;
    let t4925 = t1011 * t4924;
    let t4928 = -t3241 * t1656 / F::cast_from(108.0_f64) + t1011 * t4887 / F::cast_from(288.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t4892 * t4896 - F::cast_from(0.21437009059034868486e-3_f64) * t4899 * t4902 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t4907 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t4912 - t1011 * t4916 / F::cast_from(144.0_f64) + t1011 * t4920 / F::cast_from(216.0_f64) - F::cast_from(0.76220476654346199061e-3_f64) * t3234 + t4925 / F::cast_from(864.0_f64) + t3245 / F::cast_from(864.0_f64);
    (t4915, t4916, t4919, t4920, t4924, t4928)
}
