//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 857/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk857(t4791: f64, t4794: f64, t4798: f64, t4975: f64, t4979: f64, t4981: f64, t6961: f64, t7865: f64, t8559: f64, t8560: f64, t8592: f64, t4806: f64, t4992: f64, t4996: f64, t6002: f64, t7870: f64, t7874: f64, t7876: f64, t7878: f64, t8634: f64, t8636: f64, t8638: f64) -> (f64, f64) {
    let t9047 = t4975 - t8559 - t8560 + t4979 - t4981 - t6961 + 0.571528e-1_f64 * t7865 - t8592 - t4791 + t4794 + t4798;
    let t9051 = -t4806 + t8634 + t4992 - 0.675260332e-1_f64 * t6002 - t8636 - t8638 - 0.1350520664e0_f64 * t7870 - t7874 - t7876 + 0.2701041328e0_f64 * t7878 - t4996;
    (t9047, t9051)
}
