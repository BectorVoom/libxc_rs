//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1307/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1307<F: Float>(t101047: F, t101231: F, t101524: F, t26685: F, t26692: F, t26748: F, t27772: F, t27773: F, t28984: F, t28988: F, t4972: F, t7703: F, t93759: F, t93762: F, t96456: F, t96482: F, t96504: F) -> F {
    let t101588 = -t96456 - F::new(0.15445601851851851852e-3) * t93759 - F::new(0.15445601851851851852e-3) * t93762 + F::new(0.41703125000000000001e-2) * t7703 * t101047 - F::new(0.12356481481481481482e-2) * t26692 * t28984 - t96482 - F::new(0.27802083333333333334e-2) * t7703 * t27772 * t27773 * t4972 + F::new(0.18550940104166666667e-3) * t26685 * t101231 - F::new(0.13901041666666666667e-2) * t26748 * t28988 - F::new(0.13901041666666666667e-2) * t7703 * t101524 - F::new(0.61836467013888888889e-4) * t96504;
    t101588
}
