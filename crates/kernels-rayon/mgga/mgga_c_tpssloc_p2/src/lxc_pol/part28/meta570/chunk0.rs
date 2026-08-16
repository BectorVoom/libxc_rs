//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1850/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1850(t7524: f64, t81612: f64, t81613: f64, t4240: f64, t81865: f64, t4191: f64, t13302: f64, t23146: f64, t13322: f64, t4250: f64, t13316: f64, t13312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87177 = t81612 * t81613 * t7524;
    let t87183 = t81865 * t4240;
    let t87185 = t81865 * t4191;
    let t87187 = t23146 * t13302;
    let t87189 = t23146 * t13322;
    let t87191 = t81865 * t4250;
    let t87193 = t23146 * t13316;
    let t87195 = t23146 * t13312;
    (t87177, t87183, t87185, t87187, t87189, t87191, t87193, t87195)
}
