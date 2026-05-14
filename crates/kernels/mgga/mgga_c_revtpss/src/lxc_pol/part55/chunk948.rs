//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 948/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk948<F: Float>(t32198: F, t32273: F, t32280: F, t32285: F, t32293: F, t32674: F, t32678: F, t32681: F, t32682: F, t32683: F, t32686: F, t32690: F, t7298: F, t7304: F, t8706: F, t1419: F, t3140: F) -> (F, F) {
    let t32698 = 0.7437465841810202164e-3 * t32285 + 0.57119737665102352616e0 * t8706 * t32674 + 0.57119737665102352616e0 * t8706 * t32678 + t32681 + t32682 - t32683 - 0.3718732920905101082e-3 * t32273 - 0.17135921299530705785e1 * t8706 * t32686 + 0.8673628188205199462e0 * t32690 * t7304 + 0.7437465841810202164e-3 * t32280 + 0.14874931683620404328e-2 * t32293 - 0.56468933516960933999e-3 * t32198 + 0.17347256376410398924e1 * t32690 * t7298;
    let t32699 = t1419 * t3140;
    (t32698, t32699)
}
