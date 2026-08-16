//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1245/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1245(t10680: f64, t10681: f64, t10683: f64, t3033: f64, t10673: f64, t10674: f64, t10676: f64, t39290: f64, t42462: f64, t42465: f64, t42467: f64, t42471: f64, t42475: f64, t42814: f64, t42818: f64, t42822: f64, t42824: f64, t42826: f64, t42832: f64, t42836: f64) -> f64 {
    let t43838 = t10680 * t10681 * t3033 * t10683;
    let t43842 = t10673 * t10674 * t3033 * t10676;
    let t43844 = -t42462 + t42465 + t42467 - t42471 + t42475 + t42814 - t42818 - t42822 - t42824 + t42826 + 0.60975299583150056628e-3_f64 * t39290 - t42832 - t42836 + 0.36021158228745895953e-3_f64 * t43838 - 0.5124043883133942371e-4_f64 * t43842;
    t43844
}
