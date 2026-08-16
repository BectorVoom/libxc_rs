//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1355/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1355(t2061: f64, t4302: f64, t578: f64, t16673: f64, t4261: f64, t4260: f64, t4306: f64, t16721: f64, t4293: f64, t6010: f64, t4281: f64, t5929: f64) -> (f64, f64, f64, f64, f64) {
    let t17427 = t2061 * t4302;
    let t17428 = t578 * t17427;
    let t17430 = t4261 * t16673;
    let t17431 = t4260 * t17430;
    let t17433 = t2061 * t4306;
    let t17434 = t578 * t17433;
    let t17436 = t4293 * t16721;
    let t17437 = t6010 * t17436;
    let t17439 = t4281 * t5929;
    (t17428, t17431, t17434, t17437, t17439)
}
