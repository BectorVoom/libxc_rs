//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2048/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2048<F: Float>(t8107: F, t9593: F, t109077: F, t109104: F, t109150: F, t109153: F, t1453: F, t18245: F, t2056: F, t2108: F, t25082: F, t26405: F, t27153: F, t27833: F, t28167: F, t28196: F, t28198: F, t28588: F, t28709: F, t29506: F, t30122: F, t30138: F, t30584: F, t30612: F, t33183: F, t34495: F, t35927: F, t5627: F, t6934: F, t7235: F, t7367: F, t7374: F, t7484: F, t7537: F, t7539: F, t7898: F, t8109: F, t86815: F, t98450: F) -> F {
    let t111176 = t8107 * t9593;
    let t111214 = t7484 * t6934 + F::cast_from(4.0_f64) * t28196 * t111176 * t28198 - F::cast_from(6.0_f64) * t98450 * t28588 + t30612 * t1453 - t29506 * t7539 - F::cast_from(6.0_f64) * t25082 * t34495 * t27153 + F::cast_from(2.0_f64) * t27833 * t8109 - F::cast_from(6.0_f64) * t25082 * t33183 * t30122 - F::cast_from(2.0_f64) * t7898 * t28709 - F::cast_from(2.0_f64) * t18245 * t7374 - F::cast_from(4.0_f64) * t109150 * t2056 - F::cast_from(4.0_f64) * t109153 * t2056 - F::cast_from(4.0_f64) * t30138 * t7367 - F::cast_from(12.0_f64) * t28167 * t26405 * t109104 - t7235 * t30584 - F::cast_from(6.0_f64) * t25082 * t26405 * t86815 + F::cast_from(12.0_f64) * t28167 * t35927 * t5627 + t109077 * t2108 + t29506 * t7537;
    t111214
}
