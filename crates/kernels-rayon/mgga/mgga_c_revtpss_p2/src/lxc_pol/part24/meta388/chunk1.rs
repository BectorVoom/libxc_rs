//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1294/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1294(t24202: f64, t25042: f64, t1518: f64, t6765: f64, t118: f64, t1502: f64, t1519: f64, t18245: f64, t1843: f64, t1847: f64, t1911: f64, t22578: f64, t22634: f64, t22639: f64, t22747: f64, t22758: f64, t23094: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5884: f64, t5887: f64, t5921: f64, t651: f64, t6773: f64, t6934: f64, t7732: f64) -> (f64, f64, f64) {
    let t25043 = t24202 + t25042;
    let t25045 = t6765 * t1518;
    let t25048 = -t118 * t25043 - 3.0_f64 * t1502 * t6765 - 6.0_f64 * t1519 * t18245 - 3.0_f64 * t1843 * t5877 - 6.0_f64 * t1843 * t5884 + 3.0_f64 * t1847 * t6934 + 3.0_f64 * t1911 * t6773 - 6.0_f64 * t22578 * t651 - 2.0_f64 * t22634 * t651 - 6.0_f64 * t22639 * t508 - t22747 * t508 + t22758 * t569 + t23094 * t511 - 6.0_f64 * t25045 * t651 - 12.0_f64 * t4248 * t5887 - 6.0_f64 * t4248 * t5921 - 6.0_f64 * t5921 * t7732;
    (t25043, t25045, t25048)
}
