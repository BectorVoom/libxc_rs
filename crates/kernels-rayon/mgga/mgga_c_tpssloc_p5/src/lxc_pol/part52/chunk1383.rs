//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1383/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1383(t116135: f64, t25989: f64, t31918: f64, t7458: f64, t2314: f64, t33735: f64, t4034: f64, t1873: f64, t27858: f64, t652: f64, t33746: f64, t6997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123178 = t116135 * t25989;
    let t123180 = t7458 * t31918;
    let t123182 = t2314 * t33735;
    let t123184 = t4034 * t33735;
    let t123187 = t652 * t27858 * t1873;
    let t123189 = t33746 * t6997;
    (t123178, t123180, t123182, t123184, t123187, t123189)
}
