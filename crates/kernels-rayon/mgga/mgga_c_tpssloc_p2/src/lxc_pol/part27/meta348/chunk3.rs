//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1452/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1452(t12850: f64, t12854: f64, t12860: f64, t12861: f64, t12889: f64, t12891: f64, t12894: f64, t12895: f64, t12899: f64, t12903: f64, t12906: f64, t1877: f64, t2522: f64, t2553: f64, t4310: f64, t4314: f64, t776: f64, t868: f64, t9457: f64, t9462: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64) -> f64 {
    let t12907 = -2.0_f64 * t12854 * t1877 * t868 + 6.0_f64 * t12895 * t2522 * t776 + 12.0_f64 * t12899 * t4314 * t776 + 3.0_f64 * t2522 * t2553 * t4310 + 6.0_f64 * t12903 * t4314 + t12850 - t12860 + t12861 + t12889 + t12891 + t12894 - t12906 - t9457 + t9462 - t9469 + t9476 + t9484 - t9496 - t9715;
    t12907
}
