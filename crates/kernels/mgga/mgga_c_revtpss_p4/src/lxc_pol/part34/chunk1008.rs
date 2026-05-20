//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1008/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1008<F: Float>(t1609: F, t19330: F, t2924: F, t1622: F, t6173: F, t11452: F, t23705: F, t23451: F, t3014: F, t11574: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F, F, F, F) {
    let t23770 = t19330 * t1609;
    let t23772 = F::cast_from(0.48245938496077605201e2_f64) * t2924 * t23770;
    let t23773 = t1622 * t6173;
    let t23776 = t23705 * t11452;
    let t23785 = t23451 * t3014;
    let t23798 = -t11574 - F::cast_from(0.2283111111111111111e-1_f64) * t15189 + F::cast_from(0.11415555555555555555e-1_f64) * t18919 - F::cast_from(0.34246666666666666665e-1_f64) * t18924 + F::cast_from(0.17123333333333333333e-1_f64) * t18934 - F::cast_from(0.19025925925925925925e-1_f64) * t23479 + F::cast_from(0.68493333333333333331e-1_f64) * t23483 - F::cast_from(0.34246666666666666665e-1_f64) * t23501 - F::new(0.10274e0) * t23487 + F::new(0.10274e0) * t23505 - F::cast_from(0.17123333333333333333e-1_f64) * t23490;
    (t23772, t23773, t23776, t23785, t23798)
}
