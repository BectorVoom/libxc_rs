//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1072/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1072(t2247: f64, t4188: f64, t4248: f64, t491: f64, t1528: f64, t4254: f64) -> (f64, f64, f64, f64) {
    let t27494 = t2247 * t4188;
    let t27514 = t4248 * t491;
    let t27517 = t1528 * t491;
    let t27520 = t4254 * t491;
    (t27494, t27514, t27517, t27520)
}
