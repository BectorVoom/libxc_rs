//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 940/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk940<F: Float>(t1079: F, t1695: F, t6244: F, t11133: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F) {
    let t23583 = t1079 * t6244 * t1695;
    let t23598 = -t11133 - F::new(0.19755555555555555556e-1) * t15189 + F::new(0.9877777777777777778e-2) * t18919 - F::new(0.29633333333333333334e-1) * t18924 + F::new(0.14816666666666666667e-1) * t18934 - F::new(0.16462962962962962963e-1) * t23479 + F::new(0.59266666666666666668e-1) * t23483 - F::new(0.29633333333333333334e-1) * t23501 - F::new(0.88900000000000000002e-1) * t23487 + F::new(0.88900000000000000002e-1) * t23505 - F::new(0.14816666666666666667e-1) * t23490;
    (t23583, t23598)
}
