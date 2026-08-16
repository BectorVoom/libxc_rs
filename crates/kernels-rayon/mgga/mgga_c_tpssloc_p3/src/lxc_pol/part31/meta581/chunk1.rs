//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1821/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1821(t22892: f64, t22893: f64, t26384: f64, t26388: f64, t7733: f64, t81186: f64, t5318: f64, t552: f64, t5187: f64, t562: f64, t26392: f64, t80670: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90797 = t22892 * t22893 * t26384;
    let t90805 = t22892 * t22893 * t26388;
    let t90807 = t81186 * t7733;
    let t90809 = t552 * t5318;
    let t90818 = t562 * t5187;
    let t90837 = t80670 * t26392;
    (t90797, t90805, t90807, t90809, t90818, t90837)
}
