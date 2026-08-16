//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3190/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3190(t58598: f64, t58700: f64, t58703: f64, t58707: f64, t58711: f64, t58713: f64, t58715: f64, t58718: f64, t58720: f64, t58722: f64, t58726: f64, t57817: f64, t57912: f64, t58472: f64, t58475: f64, t58477: f64, t58479: f64, t58481: f64, t58591: f64, t58686: f64, t58688: f64, t58690: f64, t58692: f64, t58695: f64) -> f64 {
    let t58727 = -t58700 + t58703 - t58707 - t58711 + t58598 - t58713 - t58715 + t58718 - t58720 - t58722 - t58726;
    let t58730 = t57817 + t57912 + t58686 + t58472 + t58475 + t58477 + t58479 + t58481 + t58591 - t58688 + t58690 + t58692 - t58695 + t58727;
    t58730
}
