//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3391/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3391(t15386: f64, t52508: f64, t4732: f64, t52452: f64, t981: f64, t2873: f64, t6104: f64, t2876: f64, t15520: f64, t4719: f64, t19082: f64, t3022: f64) -> (f64, f64, f64, f64, f64) {
    let t63673 = 0.19298375398431042081e3_f64 * t52508 * t15386;
    let t63676 = 0.34631718211362927518e2_f64 * t981 * t4732 * t52452;
    let t63677 = t6104 * t2873;
    let t63679 = 2.0_f64 * t63677 * t2876;
    let t63681 = 0.23392894490538584828e1_f64 * t4719 * t15520;
    let t63683 = 0.70178683471615754484e1_f64 * t3022 * t19082;
    (t63673, t63676, t63679, t63681, t63683)
}
