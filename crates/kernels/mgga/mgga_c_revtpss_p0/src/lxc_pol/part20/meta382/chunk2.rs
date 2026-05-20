//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1390/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1390<F: Float>(t40392: F, t40457: F, t40520: F, t40596: F, t40671: F, t40746: F, t40811: F, t40873: F, t10661: F, t10861: F, t213: F, t234: F, t39714: F, t40298: F, t40303: F, t40307: F, t40311: F, t40314: F, t40316: F, t40318: F, t40369: F, t4366: F, t4504: F, t820: F, t879: F) -> (F, F) {
    let t40876 = t40392 + t40457 + t40520 + t40596 + t40671 + t40746 + t40811 + t40873;
    let t40886 = -F::cast_from(0.11708928647259339623e0_f64) * t40298 + F::cast_from(0.52683593463484092788e1_f64) * t4504 * t39714 * t4366 - F::cast_from(0.43902994552903410657e-1_f64) * t40303 + F::cast_from(0.21951497276451705328e-1_f64) * t40307 - F::cast_from(0.21951497276451705328e-1_f64) * t40311 - t40314 + t40316 + F::cast_from(0.44178176337912614788e-3_f64) * t40318 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t234 * t40876 + F::cast_from(0.15805078039045227836e2_f64) * t820 * t10661 * t10861 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t40369;
    (t40876, t40886)
}
