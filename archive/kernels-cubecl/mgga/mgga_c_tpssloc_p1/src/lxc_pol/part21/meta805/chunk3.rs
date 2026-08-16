//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2796/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2796<F: Float>(t232: F, t58947: F, t59072: F, t13184: F, t13193: F, t13210: F, t13251: F, t13265: F, t13302: F, t13350: F, t1510: F, t16891: F, t2643: F, t2684: F, t41116: F, t4172: F, t4180: F, t4234: F, t4250: F, t4255: F, t47039: F, t47044: F, t47047: F, t47049: F, t47079: F, t47081: F, t5619: F, t58890: F, t58900: F, t58904: F, t817: F, t819: F, t820: F, t9613: F) -> (F, F) {
    let t59074 = (t58947 + t59072) * t232;
    let t59088 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t58890 + t47044 * t4250 / F::cast_from(192.0_f64) + t13251 * t13302 / F::cast_from(192.0_f64) - t2643 * t4180 * t16891 * t2684 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t58900 + t13251 * t13210 / F::cast_from(384.0_f64) - t58904 * t13265 / F::cast_from(256.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t2643 * t13350 * t4234 * t4255 - F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t47047 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t47049 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t4172 * t13193 - t817 * t819 * t820 * t59074 / F::cast_from(3072.0_f64) - t9613 * t5619 / F::cast_from(3072.0_f64) + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t41116 + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2643 * t47039 * t1510 * t13184 + F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t47079 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t47081;
    (t59074, t59088)
}
