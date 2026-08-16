//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 805/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk805<F: Float>(t28: F, t265: F, t504: F, t28755: F, t1409: F, t1972: F, t28802: F, t52: F, t5398: F, t7664: F, t28763: F, t5161: F, t7753: F, t1983: F, t113: F, t1459: F, t1980: F, t24999: F, t27993: F, t27996: F, t28020: F, t28027: F, t28029: F, t28032: F, t28034: F, t28036: F, t28038: F, t28040: F, t28042: F, t28047: F, t28240: F, t510: F, t5460: F, t5494: F, t574: F, t6468: F, t6517: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t28803 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t28755);
    let t28810 = piecewise3::<F>(t401, t28802, t28803 * t52 / F::cast_from(2.0_f64) - t7664 * t1409 - t1972 * t5398 / F::cast_from(2.0_f64));
    let t28811 = t28763 + t28810;
    let t28813 = t7753 * t5161;
    let t28815 = F::cast_from(2.0_f64) * t1983 * t28813;
    let t28816 = -t113 * t28811 - F::cast_from(4.0_f64) * t1459 * t24999 + t1980 * t6468 - t27993 * t510 - F::cast_from(2.0_f64) * t27996 * t510 + t28020 * t574 - F::cast_from(4.0_f64) * t5460 * t6517 - F::cast_from(2.0_f64) * t5494 * t6517 - t28027 - t28029 - t28032 - t28034 - t28036 - t28038 - t28040 - t28042 - t28047 + t28240 - t28815;
    (t28811, t28813, t28816)
}
