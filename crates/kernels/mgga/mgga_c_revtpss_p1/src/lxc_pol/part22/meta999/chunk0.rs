//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3391/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3391<F: Float>(t15386: F, t52508: F, t4732: F, t52452: F, t981: F, t2873: F, t6104: F, t2876: F, t15520: F, t4719: F, t19082: F, t3022: F) -> (F, F, F, F, F) {
    let t63673 = F::cast_from(0.19298375398431042081e3_f64) * t52508 * t15386;
    let t63676 = F::cast_from(0.34631718211362927518e2_f64) * t981 * t4732 * t52452;
    let t63677 = t6104 * t2873;
    let t63679 = F::cast_from(2.0_f64) * t63677 * t2876;
    let t63681 = F::cast_from(0.23392894490538584828e1_f64) * t4719 * t15520;
    let t63683 = F::cast_from(0.70178683471615754484e1_f64) * t3022 * t19082;
    (t63673, t63676, t63679, t63681, t63683)
}
