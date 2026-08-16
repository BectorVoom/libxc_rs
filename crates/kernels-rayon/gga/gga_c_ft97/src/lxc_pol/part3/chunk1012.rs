//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1012/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1012(t17780: f64, t4206: f64, t19030: f64, t2771: f64, t17727: f64, t17732: f64, t4199: f64, t19267: f64, t10613: f64, t19271: f64, t17766: f64, t1775: f64, t5349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19669 = t4206 * t17780;
    let t19672 = t2771 * t19030;
    let t19675 = t4206 * t17727;
    let t19678 = t4199 * t17732;
    let t19681 = t2771 * t19267;
    let t19684 = t10613 * t19271;
    let t19687 = t4199 * t17766;
    let t19691 = t1775 * t5349;
    (t19669, t19672, t19675, t19678, t19681, t19684, t19687, t19691)
}
