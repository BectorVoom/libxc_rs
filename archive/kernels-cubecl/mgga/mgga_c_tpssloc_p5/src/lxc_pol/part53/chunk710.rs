//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 710/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk710<F: Float>(t25: F, t28: F, t265: F, t394: F, t504: F, t202: F, t8743: F, t8747: F, t193: F, t2752: F, t870: F, t1877: F, t40: F, t8744: F, t8748: F, t52: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8753 = t202 * t8743;
    let t8756 = t202 * t8747;
    let t8759 = -t193 * t2752 * t8756 + t193 * t870 * t8753;
    let t8760 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t8759);
    let t8763 = piecewise3::<F>(t115, t1877 * t8744 * t25 / F::cast_from(2.0_f64) - t1877 * t8748 * t25 / F::cast_from(2.0_f64), t8760 * t40 / F::cast_from(2.0_f64));
    let t8770 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t8759);
    let t8773 = piecewise3::<F>(t401, t1877 * t8744 * t28 / F::cast_from(2.0_f64) - t1877 * t8748 * t28 / F::cast_from(2.0_f64), t8770 * t52 / F::cast_from(2.0_f64));
    (t8753, t8756, t8760, t8763, t8770, t8773)
}
