//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 577/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk577(t1224: f64, t4640: f64, t4840: f64, t1697: f64, t4644: f64, t4648: f64, t4835: f64, t4838: f64, t1701: f64, t1705: f64, t1704: f64, t617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4842 = t1224 * t4840 * t4640;
    let t4845 = t1224 * t1697 * t4644;
    let t4848 = t1224 * t1697 * t4648;
    let t4850 = t4835 + 0.11872222222222222222e-1_f64 * t4838 - 0.11872222222222222222e-1_f64 * t4842 + 0.35616666666666666666e-1_f64 * t4845 - 0.17808333333333333333e-1_f64 * t4848;
    let t4853 = t1701 * t1705;
    let t4856 = t1704 * t617;
    (t4842, t4845, t4848, t4850, t4853, t4856)
}
