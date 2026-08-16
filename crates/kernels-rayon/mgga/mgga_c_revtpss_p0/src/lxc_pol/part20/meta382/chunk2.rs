//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1390/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1390(t40392: f64, t40457: f64, t40520: f64, t40596: f64, t40671: f64, t40746: f64, t40811: f64, t40873: f64, t10661: f64, t10861: f64, t213: f64, t234: f64, t39714: f64, t40298: f64, t40303: f64, t40307: f64, t40311: f64, t40314: f64, t40316: f64, t40318: f64, t40369: f64, t4366: f64, t4504: f64, t820: f64, t879: f64) -> (f64, f64) {
    let t40876 = t40392 + t40457 + t40520 + t40596 + t40671 + t40746 + t40811 + t40873;
    let t40886 = -0.11708928647259339623e0_f64 * t40298 + 0.52683593463484092788e1_f64 * t4504 * t39714 * t4366 - 0.43902994552903410657e-1_f64 * t40303 + 0.21951497276451705328e-1_f64 * t40307 - 0.21951497276451705328e-1_f64 * t40311 - t40314 + t40316 + 0.44178176337912614788e-3_f64 * t40318 + 0.65854491829355115987e0_f64 * t213 * t234 * t40876 + 0.15805078039045227836e2_f64 * t820 * t10661 * t10861 - 0.65854491829355115987e0_f64 * t820 * t879 * t40369;
    (t40876, t40886)
}
