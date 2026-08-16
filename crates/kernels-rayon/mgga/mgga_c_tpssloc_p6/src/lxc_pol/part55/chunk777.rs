//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 777/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk777(t533: f64, t7752: f64, t1390: f64, t1983: f64, t2019: f64, t5161: f64, t1873: f64, t5371: f64, t1458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7753 = t533 * t7752;
    let t7754 = t7753 * t1390;
    let t7755 = t1983 * t7754;
    let t7756 = t2019 * t5161;
    let t7757 = t1983 * t7756;
    let t7768 = 0.135e2_f64 * t5371 * t1873;
    let t7769 = t1873 * t1458;
    (t7753, t7754, t7755, t7756, t7757, t7768, t7769)
}
