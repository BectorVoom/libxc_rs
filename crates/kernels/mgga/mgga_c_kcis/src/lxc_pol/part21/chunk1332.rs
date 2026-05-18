//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1332/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1332<F: Float>(t1092: F, t1800: F, t27763: F, t3316: F, t26685: F, t96480: F, t26692: F, t26748: F, t27775: F, t27780: F, t27808: F, t7703: F, t93785: F, t95537: F, t95769: F, t96478: F, t96482: F, t96486: F, t96489: F) -> (F, F) {
    let t96498 = t1092 * t27763 * t1800 * t3316;
    let t96504 = t26685 * t96480;
    let t96506 = -F::new(0.88437037037037037034e-2) * t96478 - t96482 + F::new(0.41703125000000000001e-2) * t7703 * t95537 + F::new(0.24872916666666666666e-2) * t96486 - F::new(0.24872916666666666666e-2) * t96489 - F::new(0.23168402777777777778e-3) * t93785 + F::new(0.74138888888888888889e-2) * t26692 * t27775 + F::new(0.37069444444444444444e-2) * t26692 * t27780 + F::new(0.49745833333333333332e-2) * t96498 - F::new(0.27802083333333333334e-2) * t26748 * t27808 - F::new(0.27802083333333333334e-2) * t7703 * t95769 - F::new(0.61836467013888888888e-4) * t96504;
    (t96498, t96506)
}
