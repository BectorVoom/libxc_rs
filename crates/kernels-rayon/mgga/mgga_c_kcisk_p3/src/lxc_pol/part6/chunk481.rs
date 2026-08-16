//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 481/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk481(t4597: f64, t708: f64, t1797: f64, t574: f64, t1876: f64, t682: f64, t1849: f64, t1646: f64, t673: f64, t298: f64, t446: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4598 = t708 * t4597;
    let t4603 = t1797 * t574;
    let t4604 = t4603 * t708;
    let t4609 = t1876 * t682;
    let t4614 = t708 * t1849;
    let t4623 = t1646 * t708;
    let t4629 = t673 * t574;
    let t4636 = t298 * t446 * t569;
    (t4598, t4604, t4609, t4614, t4623, t4629, t4636)
}
