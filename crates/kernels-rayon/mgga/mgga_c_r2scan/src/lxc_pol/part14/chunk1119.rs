//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1119/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1119(t10781: f64, t7996: f64, t10810: f64, t574: f64, t8066: f64, t10697: f64, t11669: f64, t11671: f64, t11670: f64, t2124: f64, t24454: f64, t25183: f64) -> (f64, f64, f64, f64, f64) {
    let t39495 = t10781 * t7996;
    let t39499 = t574 * t10810 * t8066;
    let t39502 = t10697 * t11669 * t11671;
    let t39506 = t11670 * t2124 * t24454;
    let t39509 = t11670 * t2124 * t25183;
    (t39495, t39499, t39502, t39506, t39509)
}
