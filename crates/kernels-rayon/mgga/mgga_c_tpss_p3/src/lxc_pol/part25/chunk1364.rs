//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1364/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1364(t1692: f64, t1812: f64, t18728: f64, t18812: f64, t19681: f64, t19829: f64, t19836: f64, t20417: f64, t20514: f64, t20526: f64, t21255: f64, t21270: f64, t21659: f64, t2439: f64, t3552: f64, t580: f64, t5849: f64, t5853: f64, t6354: f64, t69804: f64, t69807: f64, t69848: f64, t69871: f64, t69891: f64, t70237: f64, t70255: f64, t70258: f64, t70286: f64) -> f64 {
    let t72242 = 3.0_f64 * t2439 * t1812 * t70286 + 3.0_f64 * t3552 * t5849 * t21255 + t1692 * t21659 * t580 / 2.0_f64 + 3.0_f64 * t18728 * t69848 + t1692 * t18812 * t70258 + 3.0_f64 * t2439 * t6354 * t19681 + 3.0_f64 / 2.0_f64 * t2439 * t5849 * t21270 + 6.0_f64 * t20417 * t69807 + 6.0_f64 * t18728 * t69804 + 2.0_f64 * t20526 * t70237 - t1692 * t5853 * t69871 / 2.0_f64 - t1692 * t20514 * t19836 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t69891 - t1692 * t5853 * t70255 + 3.0_f64 * t2439 * t6354 * t19829;
    t72242
}
