//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 734/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk734(t12236: f64, t1843: f64, t10741: f64, t10745: f64, t10747: f64, t10751: f64, t10754: f64, t10757: f64, t10759: f64, t10762: f64, t10765: f64, t10767: f64, t10769: f64, t10772: f64, t10775: f64, t10788: f64, t1841: f64) -> f64 {
    let t12318 = t1843 * t12236;
    let t12321 = -t10741 - t10745 + t10747 + t10751 + t10754 - t10757 - t10759 - t10762 - t10765 + t10767 + t10769 + t10772 + t10775 + t10788 + 0.85450291446024714263e-3_f64 * t1841 * t12318;
    t12321
}
