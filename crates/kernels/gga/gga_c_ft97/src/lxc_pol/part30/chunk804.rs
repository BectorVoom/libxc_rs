//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 804/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk804<F: Float>(t1476: F, t6386: F, t840: F, t871: F, t1882: F, t7631: F, t1901: F, t34062: F, t34067: F, t34070: F, t34074: F, t34078: F, t34083: F, t34086: F, t34091: F, t34095: F, t34099: F, t446: F) -> (F, F, F, F) {
    let t34102 = t1476 * t6386;
    let t34104 = t840 * t871 * t34102;
    let t34108 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t7631;
    let t34109 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t34062 + t446 * t34067 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t34070 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t34074 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t34078 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t34083 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t34086 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t34091 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t34095 - t446 * t34099 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t34104 - t34108;
    (t34102, t34104, t34108, t34109)
}
