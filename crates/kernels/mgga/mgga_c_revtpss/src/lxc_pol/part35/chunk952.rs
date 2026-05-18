//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 952/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk952<F: Float>(t11560: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t324: F) -> F {
    let t23811 = -t11560 - F::new(0.12361111111111111111e-1) * t15189 + F::new(0.61805555555555555556e-2) * t18919 - F::new(0.18541666666666666667e-1) * t18924 + F::new(0.92708333333333333334e-2) * t18934 - F::new(0.10300925925925925926e-1) * t23479 + F::new(0.37083333333333333333e-1) * t23483 - F::new(0.18541666666666666666e-1) * t23501 - F::new(0.55625000000000000001e-1) * t23487 + F::new(0.55625000000000000001e-1) * t23505 - F::new(0.92708333333333333333e-2) * t23490;
    let t23812 = t23811 * t324;
    t23812
}
