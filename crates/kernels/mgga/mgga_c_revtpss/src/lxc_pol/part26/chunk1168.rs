//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1168/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1168<F: Float>(t2439: F, t7398: F, t780: F, t785: F, t93134: F, t95546: F, t26435: F, t9303: F, t26440: F, t686: F, t72: F, t25375: F) -> (F, F, F, F, F) {
    let t95562 = t2439 * t785 * t7398 * t780;
    let t95567 = F::new(0.43639970290213137151e-3) * t93134 * t95546;
    let t95569 = F::new(0.26019841438354088051e-2) * t9303 * t26435;
    let t95571 = t26440 * t72 * t686;
    let t95572 = t25375 * t95571;
    (t95562, t95567, t95569, t95571, t95572)
}
