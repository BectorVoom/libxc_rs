//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 690/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk690<F: Float>(t336: F, t368: F, t4838: F, t3237: F, t532: F, t1008: F, t1581: F, t1077: F, t6: F, t386: F, t535: F, t1574: F, t1579: F) -> (F, F, F, F, F, F) {
    let t4840 = t336 * t368 * t4838;
    let t4843 = t3237 * t532;
    let t4846 = F::cast_from(0.85748036236139473944e-3_f64) * t1008 * t1581;
    let t4847 = t6 * t1077;
    let t4849 = t386 * t4847 * t535;
    let t4853 = t386 * t1574 * t1579;
    (t4840, t4843, t4846, t4847, t4849, t4853)
}
