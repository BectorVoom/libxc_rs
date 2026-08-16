//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3690/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3690<F: Float>(t1042: F, t1261: F, t17265: F, t17502: F, t17505: F, t17569: F, t20809: F, t21203: F, t3372: F, t3568: F, t3711: F, t3714: F, t5296: F, t5302: F, t5384: F, t56713: F, t5825: F, t60838: F, t69773: F, t69783: F, t69787: F, t69789: F, t69793: F, t69795: F) -> F {
    let t69805 = F::cast_from(0.57165357490759649296e-3_f64) * t17569 * t17502 - F::cast_from(0.30488190661738479624e-2_f64) * t69773 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t1042 * t20809 * t3372 + F::cast_from(0.47637797908966374414e-3_f64) * t1261 * t1042 * t5302 * t60838 + F::cast_from(0.11433071498151929859e-2_f64) * t69783 - F::cast_from(0.45732285992607719436e-2_f64) * t21203 * t17265 - F::cast_from(0.3811023832717309953e-3_f64) * t69787 + F::cast_from(0.20325460441158986416e-2_f64) * t69789 + F::cast_from(0.3811023832717309953e-3_f64) * t56713 + F::cast_from(0.28582678745379824648e-3_f64) * t69793 + F::cast_from(0.96545937095505185476e-2_f64) * t69795 * t3714 - F::cast_from(0.30488190661738479624e-2_f64) * t17505 * t17502 - F::cast_from(0.28582678745379824648e-3_f64) * t5384 * t1042 * t5296 * t5825 * t3568;
    t69805
}
