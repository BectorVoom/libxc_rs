//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1072/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1072<F: Float>(t1610: F, t1615: F, t27614: F, t6176: F, t27381: F, t27385: F, t27569: F, t27583: F, t27586: F, t27592: F, t27595: F, t27598: F, t27602: F, t27604: F, t27607: F, t7971: F, t7978: F, t7986: F) -> (F, F, F, F) {
    let t27615 = t1610 * t1615;
    let t27616 = t27614 * t27615;
    let t27617 = t6176 * t27616;
    let t27620 = F::new(0.23168402777777777778e-3) * t27583 * t27586 + F::new(0.23168402777777777778e-3) * t27583 * t27569 - F::new(0.7722800925925925926e-4) * t27592 - F::new(0.92835860883789062501e-5) * t27595 * t27598 + F::new(0.23168402777777777778e-3) * t27602 + F::new(0.23168402777777777778e-3) * t27604 + F::new(0.69505208333333333334e-3) * t27607 * t7986 + F::new(0.69505208333333333334e-3) * t27607 * t7971 + F::new(0.11607361111111111111e-2) * t27381 + F::new(0.19345601851851851852e-2) * t27385 - F::new(0.69505208333333333334e-3) * t7978 * t27617;
    (t27615, t27616, t27617, t27620)
}
