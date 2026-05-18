//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 960/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk960<F: Float>(t1668: F, t3154: F, t19572: F, t3117: F, t357: F, t15696: F, t6267: F, t23503: F, t4915: F, t11890: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F, F, F, F) {
    let t23929 = t3154 * t1668;
    let t23930 = t19572 * t23929;
    let t23931 = t3117 * t23930;
    let t23934 = t1668 * t357;
    let t23935 = t19572 * t23934;
    let t23936 = t3117 * t23935;
    let t23939 = t15696 * t6267;
    let t23945 = t4915 * t23503;
    let t23958 = -t11890 - F::new(0.11111111111111111111e-1) * t15189 + F::new(0.55555555555555555555e-2) * t18919 - F::new(0.16666666666666666667e-1) * t18924 + F::new(0.83333333333333333334e-2) * t18934 - F::new(0.92592592592592592592e-2) * t23479 + F::new(0.33333333333333333333e-1) * t23483 - F::new(0.16666666666666666666e-1) * t23501 - F::new(0.50000000000000000001e-1) * t23487 + F::new(0.50000000000000000001e-1) * t23505 - F::new(0.83333333333333333333e-2) * t23490;
    (t23931, t23936, t23939, t23945, t23958)
}
