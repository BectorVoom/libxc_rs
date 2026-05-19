//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1128/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1128<F: Float>(t1983: F, t7585: F, t7586: F, t8402: F, t30105: F, t8897: F, t1181: F, t2068: F, t33976: F, t599: F, t20433: F, t604: F) -> (F, F, F, F) {
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35485 = F::cast_from(0.14291339372689912324e-3_f64) * t35484;
    let t35486 = t30105 * t8897;
    let t35490 = t2068 * t1181 * t599 * t33976;
    let t35494 = t2068 * t1181 * t604 * t20433;
    (t35485, t35486, t35490, t35494)
}
