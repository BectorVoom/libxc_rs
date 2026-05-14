//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1204/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1204<F: Float>(t16016: F, t3117: F, t3133: F, t357: F, t4893: F, t3059: F, t4781: F, t1011: F, t11927: F, t11933: F, t15996: F, t15997: F, t16000: F, t16003: F, t16006: F, t16009: F, t16013: F, t4899: F, t4907: F, t4912: F) -> (F,) {
    let t16017 = t3117 * t16016;
    let t16020 = t3133 * t357;
    let t16021 = t4893 * t16020;
    let t16022 = t3117 * t16021;
    let t16025 = t357 * t3059;
    let t16026 = t4781 * t16025;
    let t16027 = t3117 * t16026;
    let t16034 = t15996 - t1011 * t15997 / 72.0 - t1011 * t16000 / 144.0 - t1011 * t16003 / 36.0 + t1011 * t16006 / 108.0 + t1011 * t16009 / 216.0 + 7.0 / 648.0 * t1011 * t16013 - 0.42874018118069736972e-3 * t4899 * t16017 - 0.21437009059034868486e-3 * t4899 * t16022 + 0.42874018118069736972e-3 * t11927 * t16027 + 0.22866142996303859718e-2 * t11933 * t4912 + 0.22866142996303859718e-2 * t11933 * t4907;
    (t16034,)
}
