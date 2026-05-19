//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 825/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk825<F: Float>(t10587: F, t762: F, t10575: F, t10577: F, t10580: F, t10582: F, t10584: F, t10586: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F) {
    let t10588 = t10587 * t762;
    let t10589 = F::cast_from(0.17544670867903938621e1_f64) * t10588;
    let t10590 = -t10575 + t9514 - t9517 - t9521 + t10577 + t10580 + t10582 - t10584 - t10586 - t9524 - t10589;
    (t10589, t10590)
}
