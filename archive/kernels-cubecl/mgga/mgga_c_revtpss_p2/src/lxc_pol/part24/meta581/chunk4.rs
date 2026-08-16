//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1809/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1809<F: Float>(t33: F, t265: F, t502: F, t87990: F, t91754: F, t91758: F, t91765: F, t91774: F, t1469: F, t1587: F, t1711: F, t1837: F, t22671: F, t22783: F, t23436: F, t25032: F, t504: F, t57: F, t5825: F, t6084: F, t6416: F, t6757: F, t87126: F, t89780: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t91777 = piecewise3::<F>(t503, t91754 + t91758 + t91765 + t91774, t87990);
    let t91789 = piecewise3::<F>(t400, t87990 * t33 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t23436 * t1711 + F::cast_from(3.0_f64) * t6084 * t6416 + F::cast_from(2.0_f64) * t1587 * t22783 + t265 * t89780 / F::cast_from(2.0_f64), t91777 * t57 / F::cast_from(2.0_f64) - F::cast_from(2.0_f64) * t25032 * t1469 - F::cast_from(3.0_f64) * t6757 * t5825 - F::cast_from(2.0_f64) * t1837 * t22671 - t504 * t87126 / F::cast_from(2.0_f64));
    t91789
}
