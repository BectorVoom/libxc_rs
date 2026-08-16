//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1156/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1156(t2278: f64, t3774: f64, t10027: f64, t832: f64, t2295: f64, t3801: f64, t2196: f64, t3734: f64, t2317: f64, t9929: f64, t2367: f64, t3886: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27839 = t3774 * t2278;
    let t27909 = t10027 * t832;
    let t27912 = t3801 * t2295;
    let t27937 = t3734 * t2196;
    let t27984 = t2317 * t9929;
    let t28023 = t2367 * t5939 * t3886;
    (t27839, t27909, t27912, t27937, t27984, t28023)
}
