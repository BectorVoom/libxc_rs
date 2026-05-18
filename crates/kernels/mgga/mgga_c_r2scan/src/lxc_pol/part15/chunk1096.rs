//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1096/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1096<F: Float>(t3270: F, t38364: F, t11477: F, t11481: F, t11484: F, t11488: F, t11491: F, t11494: F, t11499: F, t11503: F, t11507: F, t10614: F, t10618: F, t10621: F, t10625: F, t10629: F, t10633: F) -> (F, F) {
    let t38365 = t3270 * t38364;
    let t39149 = F::new(3.0) / F::new(2.0) * t11477;
    let t39150 = t11481 / F::new(2.0);
    let t39151 = t11484 / F::new(2.0);
    let t39152 = F::new(15.0) / F::new(8.0) * t11488;
    let t39153 = F::new(3.0) / F::new(2.0) * t11491;
    let t39154 = t11494 / F::new(2.0);
    let t39155 = F::new(3.0) / F::new(2.0) * t11499;
    let t39156 = F::new(3.0) / F::new(2.0) * t11503;
    let t39157 = F::new(3.0) / F::new(2.0) * t11507;
    let t39158 = t10614 - t39149 - t39150 + t39151 - t39152 + t39153 - t10618 + t10621 - t10625 + t10629 + t10633 - t39154 - t39155 + t39156 + t39157;
    (t38365, t39158)
}
