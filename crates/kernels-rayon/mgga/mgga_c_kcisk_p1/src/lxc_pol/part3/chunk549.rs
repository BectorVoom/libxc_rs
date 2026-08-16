//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 549/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk549(t1876: f64, t3290: f64, t4614: f64, t1877: f64, t3293: f64, t1646: f64, t708: f64, t1648: f64) -> (f64, f64, f64, f64) {
    let t4616 = t1876 * t4614 * t3290;
    let t4620 = t1876 * t1877 * t3293;
    let t4623 = t1646 * t708;
    let t4624 = t1648 * t1648;
    (t4616, t4620, t4623, t4624)
}
