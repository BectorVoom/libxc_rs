//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 881/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk881(t405: f64, t6460: f64, t394: f64, t5728: f64, t758: f64, t5939: f64, t922: f64, t918: f64, t2029: f64, t2387: f64, t3207: f64, t406: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6461 = t405 * t6460;
    let t6462 = t5728 * t394;
    let t6463 = t6461 * t6462;
    let t6464 = t758 * t6463;
    let t6467 = t5939 * t922;
    let t6468 = t918 * t6467;
    let t6470 = t2387 * t2029;
    let t6471 = t6470 * t3207;
    let t6472 = t406 * t6471;
    (t6461, t6462, t6463, t6464, t6467, t6468, t6471, t6472)
}
