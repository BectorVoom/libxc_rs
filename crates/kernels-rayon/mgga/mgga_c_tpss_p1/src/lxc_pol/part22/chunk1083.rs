//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1083/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1083(t11857: f64, t11860: f64, t11862: f64, t11865: f64, t11867: f64, t11871: f64, t11873: f64, t11876: f64, t11880: f64, t11885: f64, t11890: f64, t1992: f64, t4046: f64) -> (f64, f64) {
    let t11892 = -0.76790625e-1_f64 * t11857 - 0.1898925e1_f64 * t11860 - 0.9494625e0_f64 * t11862 + 0.3071625e0_f64 * t11865 + 0.15358125e0_f64 * t11867 + 0.49293999999999999999e0_f64 * t11871 + 0.13287407407407407408e0_f64 * t11873 - t11876 + 0.33218518518518518518e0_f64 * t11880 - 0.11958666666666666667e1_f64 * t11885 - 0.39862222222222222222e0_f64 * t11890;
    let t11894 = t4046 * t1992;
    (t11892, t11894)
}
