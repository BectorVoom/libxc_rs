//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1248/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1248(t54162: f64, t8147: f64, t2237: f64, t556: f64, t94424: f64, t18210: f64, t28402: f64, t7898: f64, t27345: f64, t8151: f64, t27348: f64, t28544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98524 = t54162 * t8147;
    let t98525 = t2237 * t98524;
    let t98530 = t94424 * t556;
    let t98537 = t18210 * t28402;
    let t98538 = t7898 * t98537;
    let t98566 = t8151 * t27345;
    let t98568 = t8151 * t27348;
    let t98570 = t28544 * t27348;
    (t98524, t98525, t98530, t98537, t98538, t98566, t98568, t98570)
}
