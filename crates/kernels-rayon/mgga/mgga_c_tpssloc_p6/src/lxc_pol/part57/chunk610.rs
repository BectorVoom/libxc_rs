//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 610/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk610(t533: f64, t7752: f64, t1390: f64, t1983: f64, t2019: f64, t5161: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1869: f64, t1976: f64, t1980: f64, t510: f64, t574: f64, t6517: f64, t652: f64, t7451: f64, t7457: f64, t7460: f64, t7463: f64, t7470: f64, t7472: f64, t7670: f64, t7681: f64, t7686: f64, t7690: f64) -> (f64, f64, f64, f64) {
    let t7753 = t533 * t7752;
    let t7754 = t7753 * t1390;
    let t7755 = t1983 * t7754;
    let t7756 = t2019 * t5161;
    let t7757 = t1983 * t7756;
    let t7758 = -t113 * t7670 - t1442 * t1976 - 2.0_f64 * t1459 * t6517 - t1774 * t1869 + t1849 * t1980 - t510 * t7451 + t574 * t7681 - 2.0_f64 * t652 * t7472 - t7457 - t7460 - t7463 - t7470 + t7686 + t7690 + t7755 - t7757;
    (t7753, t7754, t7756, t7758)
}
