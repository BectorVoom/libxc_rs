//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1968/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1968<F: Float>(t1448: F, t6922: F, t7897: F, t8995: F, t101448: F, t101451: F, t101755: F, t101756: F, t105870: F, t105873: F, t105876: F, t105878: F, t105881: F, t105883: F, t95397: F) -> (F, F, F) {
    let t109263 = t6922 * t1448;
    let t109269 = t7897 * t8995;
    let t109367 = -t95397 - t101448 - F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t101451 - t101755 + t101756 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t105870 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t105873 + t105876 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t105878 + t105881 / F::cast_from(2.0_f64) - t105883 / F::cast_from(4.0_f64);
    (t109263, t109269, t109367)
}
