//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1120/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1120(t35206: f64, t40591: f64, t3565: f64, t7400: f64, t9439: f64, t23997: f64, t26520: f64, t1882: f64, t35099: f64, t32729: f64, t35056: f64, t1060: f64, t106619: f64, t106623: f64, t106894: f64, t139820: f64, t139823: f64, t144: f64, t1901: f64, t2185: f64, t27016: f64, t27073: f64, t27096: f64, t27256: f64, t32912: f64, t33227: f64, t446: f64, t569: f64, t925: f64, t95789: f64) -> (f64, f64, f64, f64, f64) {
    let t147856 = t40591 * t35206;
    let t147866 = t9439 * t7400 * t3565;
    let t147887 = t23997 * t26520;
    let t147892 = t1882 * t35099;
    let t147894 = t32729 * t3565;
    let t147919 = t1882 * t35056;
    let t147921 = -2.0_f64 / 9.0_f64 * t147892 - t446 * t144 * t147894 / 3.0_f64 - t139820 / 27.0_f64 - t139823 + 4.0_f64 / 3.0_f64 * t446 * t2185 * t1060 * t32912 - 4.0_f64 / 3.0_f64 * t1901 * t106623 * t27016 - 2.0_f64 / 9.0_f64 * t1901 * t95789 * t27256 + 4.0_f64 / 27.0_f64 * t1901 * t106619 * t27073 - 4.0_f64 / 9.0_f64 * t1901 * t106894 * t27096 - t446 * t569 * t33227 * t925 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t147919;
    (t147856, t147866, t147887, t147894, t147921)
}
