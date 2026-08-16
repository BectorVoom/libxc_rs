//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1353/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1353(t1364: f64, t1398: f64, t14076: f64, t1692: f64, t18803: f64, t18807: f64, t18812: f64, t19809: f64, t20417: f64, t20510: f64, t2116: f64, t2439: f64, t3552: f64, t35530: f64, t3610: f64, t3724: f64, t44170: f64, t44329: f64, t44350: f64, t44470: f64, t44474: f64, t5849: f64, t5853: f64, t62820: f64, t6354: f64, t6365: f64, t63863: f64, t66281: f64, t750: f64, t821: f64) -> f64 {
    let t66704 = 3.0_f64 * t1364 * t18803 * t2439 - t1398 * t1692 * t62820 - 6.0_f64 * t14076 * t18807 * t2439 - 2.0_f64 * t1692 * t18807 * t3724 + 4.0_f64 * t1692 * t18812 * t44350 - 2.0_f64 * t1692 * t66281 * t821 - 6.0_f64 * t18807 * t19809 * t2439 + 6.0_f64 * t18812 * t2439 * t44474 + 6.0_f64 * t20510 * t2439 * t750 + 6.0_f64 * t2116 * t3552 * t6354 + 6.0_f64 * t2439 * t3610 * t5849 - 6.0_f64 * t2439 * t44329 * t5853 - 6.0_f64 * t2439 * t44470 * t5853 - 6.0_f64 * t3552 * t5853 * t63863 - 12.0_f64 * t20417 * t44170 + 6.0_f64 * t35530 * t6365;
    t66704
}
