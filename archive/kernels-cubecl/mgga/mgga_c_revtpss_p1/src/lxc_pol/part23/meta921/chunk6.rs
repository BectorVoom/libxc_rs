//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2977/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2977<F: Float>(t11922: F, t11927: F, t23838: F, t23998: F, t3115: F, t1042: F, t1063: F, t11656: F, t11672: F, t15618: F, t15935: F, t16208: F, t1675: F, t19878: F, t19944: F, t20079: F, t23839: F, t23848: F, t23917: F, t3127: F, t43297: F, t4801: F, t4806: F, t51958: F, t51963: F, t66784: F, t78570: F, t78750: F, t78756: F, t78763: F, t78765: F, t78770: F, t78785: F, t78790: F) -> F {
    let t78802 = t11927 * t11922 * t23838;
    let t78805 = t3115 * t11922 * t23998;
    let t78807 = F::cast_from(0.42344709252414555035e-3_f64) * t78750 + F::cast_from(0.42874018118069736972e-3_f64) * t15618 * t20079 - F::cast_from(0.38110238327173099531e-2_f64) * t11672 * t23917 + F::cast_from(0.47637797908966374413e-3_f64) * t78756 + F::cast_from(0.25724410870841842184e-2_f64) * t19878 * t19944 - F::cast_from(0.22866142996303859718e-2_f64) * t66784 * t1675 + F::cast_from(0.47637797908966374414e-3_f64) * t78763 + F::cast_from(0.85748036236139473944e-3_f64) * t3127 * t1042 * t4801 * t78765 - F::cast_from(0.28582678745379824648e-3_f64) * t1063 * t1042 * t4801 * t78770 + F::cast_from(0.23818898954483187207e-3_f64) * t1063 * t1042 * t4806 * t78770 - F::cast_from(0.63517063878621832552e-3_f64) * t3127 * t1042 * t16208 * t78570 + F::cast_from(0.38110238327173099531e-2_f64) * t11656 * t23848 - F::cast_from(0.34299214494455789578e-2_f64) * t1063 * t1042 * t51958 * t78785 + F::cast_from(0.25724410870841842183e-2_f64) * t1063 * t1042 * t15935 * t78790 + F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t1042 * t51963 * t78785 - F::cast_from(0.68598428988911579157e-2_f64) * t43297 * t23839 + F::cast_from(0.85748036236139473947e-3_f64) * t78802 - F::cast_from(0.42874018118069736972e-3_f64) * t78805;
    t78807
}
