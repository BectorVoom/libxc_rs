//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1182/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1182(t18506: f64, t19146: f64, t19549: f64, t19624: f64, t19670: f64, t19732: f64, t19775: f64, t19821: f64, t393: f64, t1141: f64, t6634: f64, t1203: f64) -> (f64, f64) {
    let t19824 = t18506 + t19146 + t19549 + t19624 + t19670 + t19732 + t19775 + t19821;
    let t19825 = t19824 * t393;
    let t19826 = t6634 * t1141;
    let t19827 = t19826 * t1203;
    (t19825, t19827)
}
