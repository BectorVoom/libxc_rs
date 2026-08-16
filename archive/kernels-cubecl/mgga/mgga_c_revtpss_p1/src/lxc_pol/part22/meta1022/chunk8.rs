//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3569/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3569<F: Float>(t30: F, t265: F, t393: F, t63193: F, t63587: F, t63629: F, t63671: F, t63899: F, t63938: F, t64513: F, t64532: F, t68211: F, t1106: F, t13312: F, t1468: F, t1469: F, t15083: F, t16618: F, t1704: F, t18280: F, t18281: F, t18884: F, t20236: F, t2257: F, t2258: F, t2838: F, t3340: F, t395: F, t4186: F, t45: F, t5028: F, t51835: F, t5824: F, t5825: F, t605: F, t606: F, t60754: F, t6084: F, t63202: F, t63204: F, t63206: F, t6405: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t68215 = piecewise3::<F>(t394, t63587 + t63629 + t63671 + t63899 + t63938 + t64513 + t64532 + t68211, t63193);
    let t68231 = piecewise3::<F>(t120, t63193 * t30 / F::cast_from(2.0_f64) + t18884 * t605 + t6084 * t2257 / F::cast_from(2.0_f64) + t15083 * t1468 + t63202 + t63204 - t63206 + t2838 * t5824 / F::cast_from(2.0_f64) + t895 * t18280 + t51835, t68215 * t45 / F::cast_from(2.0_f64) + t20236 * t606 + t6405 * t2258 / F::cast_from(2.0_f64) + t16618 * t1469 + F::cast_from(2.0_f64) * t5028 * t4186 + t1704 * t13312 + t3340 * t5825 / F::cast_from(2.0_f64) + t1106 * t18281 + t395 * t60754 / F::cast_from(2.0_f64));
    t68231
}
