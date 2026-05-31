//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1083/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1083<F: Float>(t24202: F, t25042: F, t1518: F, t6765: F, t118: F, t1502: F, t1519: F, t18245: F, t1843: F, t1847: F, t1911: F, t22578: F, t22634: F, t22639: F, t22747: F, t22758: F, t23094: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5884: F, t5887: F, t5921: F, t651: F, t6773: F, t6934: F, t7732: F) -> (F, F, F) {
    let t25043 = t24202 + t25042;
    let t25045 = t6765 * t1518;
    let t25048 = -t118 * t25043 - F::cast_from(3.0_f64) * t1502 * t6765 - F::cast_from(6.0_f64) * t1519 * t18245 - F::cast_from(3.0_f64) * t1843 * t5877 - F::cast_from(6.0_f64) * t1843 * t5884 + F::cast_from(3.0_f64) * t1847 * t6934 + F::cast_from(3.0_f64) * t1911 * t6773 - F::cast_from(6.0_f64) * t22578 * t651 - F::cast_from(2.0_f64) * t22634 * t651 - F::cast_from(6.0_f64) * t22639 * t508 - t22747 * t508 + t22758 * t569 + t23094 * t511 - F::cast_from(6.0_f64) * t25045 * t651 - F::cast_from(12.0_f64) * t4248 * t5887 - F::cast_from(6.0_f64) * t4248 * t5921 - F::cast_from(6.0_f64) * t5921 * t7732;
    (t25043, t25045, t25048)
}
