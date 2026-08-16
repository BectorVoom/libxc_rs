//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3751/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3751(t1222: f64, t17240: f64, t20306: f64, t12772: f64, t21156: f64, t3625: f64, t12866: f64, t17456: f64, t17639: f64, t17645: f64, t17661: f64, t44823: f64, t44829: f64, t44838: f64, t44884: f64, t5308: f64, t5312: f64, t59162: f64, t68269: f64, t68273: f64, t68317: f64, t68328: f64) -> f64 {
    let t71377 = t1222 * t17240 * t20306;
    let t71400 = t3625 * t12772 * t21156;
    let t71406 = -t71377 / 72.0_f64 - t1222 * t5308 * t68273 / 72.0_f64 - t1222 * t5308 * t68317 / 48.0_f64 - t1222 * t5308 * t68269 / 12.0_f64 + 0.28582678745379824648e-3_f64 * t12866 * t17661 * t17639 + 0.57165357490759649296e-3_f64 * t12866 * t17661 * t17645 + t1222 * t5312 * t68328 / 6.0_f64 - 0.17149607247227894789e-2_f64 * t59162 * t17456 - 0.19055119163586549765e-3_f64 * t71400 + 0.47637797908966374413e-4_f64 * t44823 - 0.1270341277572436651e-3_f64 * t44829 - 0.95275595817932748826e-4_f64 * t44838 + 0.95275595817932748826e-4_f64 * t44884;
    t71406
}
