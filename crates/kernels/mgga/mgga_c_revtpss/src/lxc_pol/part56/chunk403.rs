//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 403/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk403<F: Float>(t1941: F, t533: F, t816: F, t546: F, t64: F, t213: F, t552: F) -> (F, F, F) {
    let t2016 = t1941 * t533 * t816;
    let t2018 = t546 * t64;
    let t2019 = t213 * t2018;
    let t2020 = t2019 * t552;
    let t2022 = t2016 / F::new(96.0) + F::new(0.42874018118069736972e-3) * t2020;
    (t2018, t2019, t2022)
}
