//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 999/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk999<F: Float>(t1983: F, t7585: F, t7586: F, t8402: F, t30105: F, t8897: F, t1181: F, t2068: F, t33976: F, t599: F, t20433: F, t604: F, t30268: F, t8783: F, t31254: F, t1479: F, t429: F) -> (F, F, F, F, F, F, F) {
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35485 = 0.14291339372689912324e-3 * t35484;
    let t35486 = t30105 * t8897;
    let t35490 = t2068 * t1181 * t599 * t33976;
    let t35494 = t2068 * t1181 * t604 * t20433;
    let t35496 = t30268 * t8783;
    let t35497 = 0.94344276868812456204e-2 * t35496;
    let t35499 = 0.85748036236139473944e-3 * t31254;
    let t35500 = t429 * t1479;
    (t35485, t35486, t35490, t35494, t35497, t35499, t35500)
}
