//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1452/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1452(t16524: f64, t31817: f64, t12524: f64, t33659: f64, t31814: f64, t2039: f64, t26135: f64, t3941: f64, t20173: f64, t33656: f64, t1873: f64, t27170: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122817 = 27.0_f64 * t16524 * t31817;
    let t122824 = 27.0_f64 * t12524 * t33659;
    let t122826 = 27.0_f64 * t16524 * t31814;
    let t122829 = 27.0_f64 * t3941 * t2039 * t26135;
    let t122831 = 27.0_f64 * t20173 * t33656;
    let t122834 = 27.0_f64 * t3941 * t27170 * t1873;
    (t122817, t122824, t122826, t122829, t122831, t122834)
}
