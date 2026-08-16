//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1461/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1461<F: Float>(t28: F, t106717: F, t109953: F, t1409: F, t20217: F, t2161: F, t29840: F, t52: F, t5398: F, t8097: F, t106747: F, t106753: F, t106756: F, t106889: F, t106891: F, t106895: F, t106899: F, t106901: F, t106905: F, t106919: F, t106964: F, t109029: F, t109055: F, t113: F, t1458: F, t1774: F, t20293: F, t20702: F, t2165: F, t29486: F, t29501: F, t29848: F, t29855: F, t4028: F, t510: F, t652: F, t7266: F, t7458: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t109963 = piecewise3::<F>(t401, t106717, t109953 * t52 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t29840 * t1409 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t8097 * t5398 - t2161 * t20217 / F::cast_from(2.0_f64));
    let t109966 = -t106747 - t109029 * t510 - F::cast_from(6.0_f64) * t4028 * t29855 + t106753 + t106756 - F::cast_from(6.0_f64) * t652 * t29848 * t1458 - t20293 * t2165 - F::cast_from(6.0_f64) * t7266 * t20702 - t106889 - t106891 - t106895 - t106899 - t106901 - t106905 - F::cast_from(6.0_f64) * t7458 * t29855 - F::cast_from(12.0_f64) * t4028 * t29501 - F::cast_from(3.0_f64) * t29486 * t1774 - t106919 - t106964 - t113 * (t109055 + t109963);
    t109966
}
