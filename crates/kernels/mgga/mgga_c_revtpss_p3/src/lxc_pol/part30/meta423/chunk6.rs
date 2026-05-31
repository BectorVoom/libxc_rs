//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1605/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1605<F: Float>(t15140: F, t16012: F, t15780: F, t4900: F, t3117: F, t3133: F, t357: F, t4893: F, t3059: F, t4781: F, t1011: F, t11927: F, t11933: F, t15996: F, t15997: F, t16000: F, t16003: F, t16006: F, t16009: F, t4899: F, t4907: F, t4912: F) -> F {
    let t16013 = t16012 * t15140;
    let t16016 = t15780 * t4900;
    let t16017 = t3117 * t16016;
    let t16020 = t3133 * t357;
    let t16021 = t4893 * t16020;
    let t16022 = t3117 * t16021;
    let t16025 = t357 * t3059;
    let t16026 = t4781 * t16025;
    let t16027 = t3117 * t16026;
    let t16034 = t15996 - t1011 * t15997 / F::cast_from(72.0_f64) - t1011 * t16000 / F::cast_from(144.0_f64) - t1011 * t16003 / F::cast_from(36.0_f64) + t1011 * t16006 / F::cast_from(108.0_f64) + t1011 * t16009 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1011 * t16013 - F::cast_from(0.42874018118069736972e-3_f64) * t4899 * t16017 - F::cast_from(0.21437009059034868486e-3_f64) * t4899 * t16022 + F::cast_from(0.42874018118069736972e-3_f64) * t11927 * t16027 + F::cast_from(0.22866142996303859718e-2_f64) * t11933 * t4912 + F::cast_from(0.22866142996303859718e-2_f64) * t11933 * t4907;
    t16034
}
