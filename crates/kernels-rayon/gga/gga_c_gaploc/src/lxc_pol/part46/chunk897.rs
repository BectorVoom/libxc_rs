//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 897/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk897(t1358: f64, t41993: f64, t6507: f64, t2339: f64, t31731: f64, t105: f64, t123: f64, t12794: f64, t12815: f64, t169: f64, t172: f64, t380: f64, t41801: f64, t419: f64, t42123: f64, t42130: f64, t42529: f64, t42533: f64, t42537: f64, t42540: f64, t42544: f64, t42547: f64, t42551: f64, t42570: f64, t452: f64, t488: f64, t492: f64) -> f64 {
    let t42573 = 0.63233348079280332442e-2_f64 * t1358 * t6507 * t41993;
    let t42575 = t1358 * t31731 * t2339;
    let t42577 = -0.31616674039640166221e-2_f64 * t1358 * t41801 * t123 * t488 + 0.23712505529730124666e-2_f64 * t42529 - 0.63233348079280332442e-2_f64 * t42533 + t42537 + t42540 + t42544 - t42547 - t42551 + 0.28455006635676149599e-1_f64 * t419 * t12815 + 0.28455006635676149599e-1_f64 * t105 * t452 * t42123 * t169 * t172 - 0.28455006635676149599e-1_f64 * t105 * t492 * t42130 - 0.28455006635676149599e-1_f64 * t419 * t12794 - 0.37940008847568199465e-1_f64 * t380 * t12794 + 0.37940008847568199465e-1_f64 * t380 * t12815 - t42570 - t42573 + 0.18970004423784099733e-1_f64 * t42575;
    t42577
}
