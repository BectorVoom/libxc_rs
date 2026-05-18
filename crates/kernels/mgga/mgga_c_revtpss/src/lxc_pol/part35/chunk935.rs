//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 935/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk935<F: Float>(t23499: F, t2908: F, t141: F, t23503: F, t930: F, t15123: F, t15189: F, t23472: F, t23476: F, t23479: F, t23483: F, t23487: F, t23490: F, t23493: F, t23496: F, t23501: F, t23505: F) -> (F, F, F) {
    let t23507 = t2908 * t23499;
    let t23508 = t141 * t23507;
    let t23510 = t930 * t23503;
    let t23511 = t141 * t23510;
    let t23514 = -F::new(0.36514074074074074075e-1) * t23472 - F::new(0.82156666666666666667e-1) * t23476 - F::new(0.33218518518518518518e0) * t23479 + F::new(0.11958666666666666667e1) * t23483 - F::new(0.17938e1) * t23487 - F::new(0.29896666666666666667e0) * t23490 + F::new(0.16431333333333333333e0) * t23493 - F::new(0.49293999999999999999e0) * t23496 - F::new(0.27385555555555555556e0) * t15123 - F::new(0.59793333333333333333e0) * t23501 + F::new(0.17938e1) * t23505 - F::new(0.82156666666666666668e-1) * t23508 + F::new(0.49293999999999999999e0) * t23511 - F::new(0.39862222222222222223e0) * t15189;
    (t23508, t23511, t23514)
}
