//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1252/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1252(t31570: f64, t32866: f64, t32867: f64, t35764: f64, t35790: f64, t37714: f64, t37717: f64, t37718: f64, t37721: f64, t37722: f64, t37723: f64, t40145: f64, t40147: f64, t40152: f64, t40156: f64, t40158: f64, t40163: f64) -> f64 {
    let t41973 = 0.62896184579208304138e-3_f64 * t31570 - t35764 - t32866 - t32867 + t37714 + t37717 + t37718 + 0.34299214494455789578e-2_f64 * t35790 - t37721 + t37722 + t37723 + 0.34299214494455789578e-2_f64 * t40145 + 7.0_f64 / 72.0_f64 * t40147 + 0.21437009059034868486e-3_f64 * t40152 + 0.14291339372689912324e-3_f64 * t40156 - 0.62896184579208304138e-3_f64 * t40158 - 0.41930789719472202759e-3_f64 * t40163;
    t41973
}
