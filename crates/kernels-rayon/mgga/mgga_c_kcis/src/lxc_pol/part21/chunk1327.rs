//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1327/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1327(t1020: f64, t26671: f64, t4796: f64, t27864: f64, t2822: f64, t13440: f64, t27763: f64, t3205: f64, t95664: f64, t27836: f64, t3213: f64, t3245: f64, t8057: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96399 = t1020 * t26671 * t4796;
    let t96401 = t2822 * t27864;
    let t96402 = 0.22109259259259259258e-2_f64 * t96401;
    let t96404 = t1020 * t27763 * t13440;
    let t96407 = t1020 * t95664 * t3205;
    let t96410 = t1020 * t27836 * t3213;
    let t96412 = t3245 * t8057;
    (t96399, t96401, t96402, t96404, t96407, t96410, t96412)
}
