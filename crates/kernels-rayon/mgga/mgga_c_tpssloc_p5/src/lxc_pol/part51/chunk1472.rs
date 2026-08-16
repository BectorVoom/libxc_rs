//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1472/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1472(t1851: f64, t8660: f64, t2098: f64, t7774: f64, t33627: f64, t580: f64, t1858: f64, t8646: f64, t116014: f64, t116028: f64, t116036: f64, t120857: f64, t122765: f64, t122774: f64, t122797: f64, t122820: f64, t122847: f64, t1398: f64, t2029: f64, t2099: f64, t2105: f64, t26510: f64, t26555: f64, t27241: f64, t3: f64, t5381: f64, t7020: f64, t7223: f64, t7946: f64, t8647: f64) -> f64 {
    let t122852 = t1851 * t8660;
    let t122853 = t2098 * t7774;
    let t122856 = t33627 * t580;
    let t122857 = t8646 * t1858;
    let t122858 = t7946 * t7020 + t120857 + t2099 * t26555 + t27241 * t2029 + t26510 * t2105 + t8647 * t5381 + t116014 + t1398 * (t122774 + t122797 + t122820 + t122847) + t7223 * t7774 + t116028 + t116036 + t122852 + t122853 + t3 * t122765 * t580 + t122856 + t122857;
    t122858
}
