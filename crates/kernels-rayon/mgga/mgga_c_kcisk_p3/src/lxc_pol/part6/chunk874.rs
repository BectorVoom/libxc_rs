//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 874/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk874(t23080: f64, t2487: f64, t2477: f64, t7718: f64, t23096: f64, t6734: f64, t8536: f64, t8500: f64, t11480: f64, t6746: f64, t16099: f64, t1856: f64, t28385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28579 = t23080 * t2487;
    let t28582 = t2477 * t7718;
    let t28585 = t23096 * t2487;
    let t28588 = t6734 * t8536;
    let t28591 = t8500 * t2487;
    let t28592 = t11480 * t28591;
    let t28595 = t6746 * t8536;
    let t28598 = t16099 * t8500;
    let t28610 = t1856 * t28385;
    (t28579, t28582, t28585, t28588, t28591, t28592, t28595, t28598, t28610)
}
