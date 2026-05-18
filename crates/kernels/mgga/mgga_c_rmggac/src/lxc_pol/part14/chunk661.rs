//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 661/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk661<F: Float>(t3851: F, t8625: F, t3814: F, t8631: F, t854: F, t8700: F, t851: F, t8704: F, t7625: F, t7628: F, t7639: F, t7646: F, t7651: F, t7656: F, t7663: F) -> F {
    let t8784 = t3851 * t8625;
    let t8786 = t3814 * t8631;
    let t8788 = t854 * t8700;
    let t8790 = t851 * t8704;
    let t8792 = -F::new(0.10620923284048465071e-2) * t7625 - t7628 - F::new(0.90915538847484472431e-2) * t7639 + F::new(0.12122071846331262991e-1) * t7646 - F::new(0.2419210303588817044e-3) * t7651 + F::new(0.28224120208536198847e-3) * t7656 + F::new(0.2993560425465952141e-1) * t8784 - F::new(0.5987120850931904282e-1) * t8786 - t7663 + F::new(0.39828462315181744016e-3) * t8788 - F::new(0.33190385262651453347e-3) * t8790;
    t8792
}
