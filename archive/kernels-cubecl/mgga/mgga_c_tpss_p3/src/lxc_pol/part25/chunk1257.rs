//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1257/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1257<F: Float>(t30: F, t259: F, t379: F, t21701: F, t1289: F, t1819: F, t21677: F, t45: F, t4579: F, t6374: F, t1812: F, t21485: F, t1497: F, t1692: F, t18728: F, t18812: F, t20514: F, t21492: F, t21495: F, t21499: F, t21510: F, t21513: F, t21516: F, t21659: F, t2439: F, t33: F, t3552: F, t5059: F, t5853: F, t6207: F, t6214: F, t6354: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t21702 = piecewise3::<F>(t380, F::cast_from(0.0_f64), t21701);
    let t21709 = piecewise3::<F>(t120, t21677, t21702 * t45 / F::cast_from(2.0_f64) + t6374 * t1289 + t1819 * t4579 / F::cast_from(2.0_f64));
    let t21710 = t1812 * t21485;
    let t21741 = F::cast_from(3.0_f64) * t3552 * t21710 + F::cast_from(3.0_f64) * t2439 * t6354 * t6207 - F::cast_from(3.0_f64) * t18728 * t21492 + F::cast_from(3.0_f64) * t2439 * t1812 * t21495 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2439 * t1812 * t21499 + t1692 * t21659 * t33 / F::cast_from(2.0_f64) - t1692 * t20514 * t6214 + t1692 * t6354 * t1497 + t1692 * t18812 * t21510 - t1692 * t5853 * t21513 - t1692 * t5853 * t21516 / F::cast_from(2.0_f64) + t1692 * t1812 * t5059 / F::cast_from(2.0_f64);
    (t21702, t21709, t21710, t21741)
}
