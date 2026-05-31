//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1120/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1120<F: Float>(t35206: F, t40591: F, t3565: F, t7400: F, t9439: F, t23997: F, t26520: F, t1882: F, t35099: F, t32729: F, t35056: F, t1060: F, t106619: F, t106623: F, t106894: F, t139820: F, t139823: F, t144: F, t1901: F, t2185: F, t27016: F, t27073: F, t27096: F, t27256: F, t32912: F, t33227: F, t446: F, t569: F, t925: F, t95789: F) -> (F, F, F, F, F) {
    let t147856 = t40591 * t35206;
    let t147866 = t9439 * t7400 * t3565;
    let t147887 = t23997 * t26520;
    let t147892 = t1882 * t35099;
    let t147894 = t32729 * t3565;
    let t147919 = t1882 * t35056;
    let t147921 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t147892 - t446 * t144 * t147894 / F::cast_from(3.0_f64) - t139820 / F::cast_from(27.0_f64) - t139823 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t1060 * t32912 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t106623 * t27016 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t95789 * t27256 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t106619 * t27073 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t106894 * t27096 - t446 * t569 * t33227 * t925 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t147919;
    (t147856, t147866, t147887, t147894, t147921)
}
