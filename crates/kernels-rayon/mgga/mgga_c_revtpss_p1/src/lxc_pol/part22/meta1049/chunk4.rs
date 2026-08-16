//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3690/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3690(t1042: f64, t1261: f64, t17265: f64, t17502: f64, t17505: f64, t17569: f64, t20809: f64, t21203: f64, t3372: f64, t3568: f64, t3711: f64, t3714: f64, t5296: f64, t5302: f64, t5384: f64, t56713: f64, t5825: f64, t60838: f64, t69773: f64, t69783: f64, t69787: f64, t69789: f64, t69793: f64, t69795: f64) -> f64 {
    let t69805 = 0.57165357490759649296e-3_f64 * t17569 * t17502 - 0.30488190661738479624e-2_f64 * t69773 + 0.14291339372689912324e-3_f64 * t3711 * t1042 * t20809 * t3372 + 0.47637797908966374414e-3_f64 * t1261 * t1042 * t5302 * t60838 + 0.11433071498151929859e-2_f64 * t69783 - 0.45732285992607719436e-2_f64 * t21203 * t17265 - 0.3811023832717309953e-3_f64 * t69787 + 0.20325460441158986416e-2_f64 * t69789 + 0.3811023832717309953e-3_f64 * t56713 + 0.28582678745379824648e-3_f64 * t69793 + 0.96545937095505185476e-2_f64 * t69795 * t3714 - 0.30488190661738479624e-2_f64 * t17505 * t17502 - 0.28582678745379824648e-3_f64 * t5384 * t1042 * t5296 * t5825 * t3568;
    t69805
}
