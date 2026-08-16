//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1790/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1790<F: Float>(t30: F, t265: F, t393: F, t28522: F, t1469: F, t2078: F, t28491: F, t4186: F, t45: F, t606: F, t7449: F, t8040: F, t1113: F, t1711: F, t1940: F, t2071: F, t2403: F, t26425: F, t26585: F, t27764: F, t27770: F, t27773: F, t27777: F, t27793: F, t27800: F, t27802: F, t27806: F, t27810: F, t27817: F, t28291: F, t28456: F, t28460: F, t28472: F, t28490: F, t33: F, t7200: F, t7207: F, t7428: F, t7432: F, t7862: F, t7869: F, t8020: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28523 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t28522);
    let t28530 = piecewise3::<F>(t120, t28491, t7449 * t1469 / F::cast_from(2.0_f64) + t2078 * t4186 / F::cast_from(2.0_f64) + t28523 * t45 / F::cast_from(2.0_f64) + t8040 * t606 / F::cast_from(2.0_f64));
    let t28577 = F::cast_from(3.0_f64) * t28291 * t27764 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t7428 * t7862 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t27770 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t27773 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t27777 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8020 * t7200 + t1940 * t28456 * t33 / F::cast_from(2.0_f64) - t1940 * t28460 * t7207 / F::cast_from(2.0_f64) + t1940 * t8020 * t1113 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t27793 - t1940 * t26585 * t7869 / F::cast_from(2.0_f64) + t28472 * t27800 - t1940 * t7432 * t27802 / F::cast_from(2.0_f64) - t1940 * t7432 * t27806 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t27810 + t1940 * t7428 * t1711 / F::cast_from(2.0_f64) - t1940 * t7432 * t27817 / F::cast_from(2.0_f64) - t28490;
    (t28523, t28530, t28577)
}
