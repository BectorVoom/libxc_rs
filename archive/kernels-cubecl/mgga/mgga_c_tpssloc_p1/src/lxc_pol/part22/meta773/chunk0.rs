//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2644/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2644<F: Float>(t28: F, t265: F, t504: F, t68418: F, t71222: F, t71252: F, t72059: F, t72074: F, t72077: F, t72078: F, t72099: F, t73931: F, t1081: F, t1260: F, t1409: F, t1534: F, t1649: F, t16558: F, t17133: F, t1768: F, t18196: F, t19276: F, t20217: F, t20390: F, t21076: F, t22414: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t5398: F, t5966: F, t607: F, t6279: F, t67060: F, t68427: F, t71090: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t73935 = piecewise3::<F>(t505, t71222 + t71252 + t72059 + t72074 + t72077 + t72078 + t72099 + t73931, t68418);
    let t73953 = piecewise3::<F>(t401, t68418 * t28 / F::cast_from(2.0_f64) + t21076 * t1081 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t17133 * t1649 - t68427 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t4324 * t5966 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1534 * t18196 + t873 * t20390 / F::cast_from(2.0_f64) + t265 * t71090 / F::cast_from(2.0_f64), t73935 * t52 / F::cast_from(2.0_f64) - t22414 * t607 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t19276 * t1409 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6279 * t3966 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5099 * t5398 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1768 * t16558 - t1260 * t20217 / F::cast_from(2.0_f64) - t506 * t67060 / F::cast_from(2.0_f64));
    t73953
}
