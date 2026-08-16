//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1021/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1021(t30817: f64, t8793: f64, t1313: f64, t30598: f64, t721: f64, t1322: f64, t7859: f64, t31612: f64, t31619: f64, t31625: f64, t31627: f64, t31629: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35876 = t30817 * t8793;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    let t35890 = 0.17149607247227894789e-2_f64 * t31612;
    let t35891 = 0.18868855373762491241e-1_f64 * t31619;
    let t35893 = 0.25724410870841842184e-2_f64 * t31625;
    let t35894 = 0.51448821741683684368e-2_f64 * t31627;
    let t35898 = 0.12862205435420921092e-1_f64 * t31629;
    (t35876, t35882, t35885, t35890, t35891, t35893, t35894, t35898)
}
