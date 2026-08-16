//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1074/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1074<F: Float>(t1882: F, t35636: F, t35596: F, t35601: F, t110401: F, t14127: F, t141817: F, t141820: F, t141834: F, t141850: F, t141852: F, t150133: F, t150233: F, t151051: F, t151387: F, t1901: F, t242: F, t24429: F, t24668: F, t2574: F, t265: F, t28023: F, t28108: F, t28145: F, t35323: F, t3821: F, t3972: F, t446: F, t6088: F, t6194: F, t6852: F, t6861: F, t729: F, t7484: F, t7560: F, t762: F, t773: F) -> F {
    let t151712 = t1882 * t35636;
    let t151715 = t1882 * t35596;
    let t151725 = t1882 * t35601;
    let t151760 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t14127 * t24668 * t28108 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t110401 * t28145 + t141817 / F::cast_from(9.0_f64) - t141820 - t151712 / F::cast_from(9.0_f64) - t141834 / F::cast_from(27.0_f64) - t151715 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t151387 + t446 * t729 * t762 * t7484 * t3972 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t151725 - t446 * t242 * t151051 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t28023 * t6088 - t446 * t729 * t7560 * t3821 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t773 * t35323 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t265 * t150133 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t6194 * t6852 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t265 * t150233 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t141850 + t141852 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t24429 * t6861;
    t151760
}
