//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 777/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk777(t1717: f64, t621: f64, t588: f64, t2706: f64, t639: f64, t1095: f64, t5873: f64, t1083: f64, t5804: f64, t1979: f64, t2848: f64, t1107: f64, t5493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7126 = t1717 * t621;
    let t7143 = t588 * t621;
    let t7201 = t2706 * t639;
    let t7247 = t1095 * t5873;
    let t7285 = t1083 * t5804;
    let t7299 = t2848 * t1979;
    let t7308 = t1107 * t5493;
    (t7126, t7143, t7201, t7247, t7285, t7299, t7308)
}
