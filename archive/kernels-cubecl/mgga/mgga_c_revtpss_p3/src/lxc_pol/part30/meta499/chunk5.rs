//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1861/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1861<F: Float>(t1252: F, t1266: F, t26849: F, t26852: F, t26855: F, t26863: F, t26867: F, t26870: F, t26873: F, t26877: F, t26880: F, t3591: F, t3613: F, t3620: F, t3631: F, t3640: F, t3644: F, t3714: F, t3723: F, t7618: F, t7624: F) -> F {
    let t26883 = -F::cast_from(0.42874018118069736972e-3_f64) * t26849 * t3613 - F::cast_from(0.57165357490759649296e-3_f64) * t26852 * t1266 - F::cast_from(0.3811023832717309953e-3_f64) * t26855 - F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t3640 - F::cast_from(0.57165357490759649296e-3_f64) * t7624 * t3644 + F::cast_from(0.47637797908966374413e-3_f64) * t7624 * t3620 + F::cast_from(0.57165357490759649296e-3_f64) * t26863 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t3631 - F::cast_from(0.85748036236139473944e-3_f64) * t26870 * t3723 + F::cast_from(0.85748036236139473944e-3_f64) * t26873 * t1252 - t26877 + F::cast_from(0.42874018118069736972e-3_f64) * t7618 * t3591 + F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t3714;
    t26883
}
