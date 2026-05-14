//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 696/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk696<F: Float>(t1882: F, t7495: F, t7499: F, t242: F, t33264: F, t10002: F, t7546: F, t33651: F, t33654: F, t33658: F, t33660: F, t33665: F, t33668: F, t33673: F, t33676: F, t446: F) -> (F, F, F, F, F, F) {
    let t33680 = 2.0 / 9.0 * t1882 * t7495;
    let t33682 = 2.0 / 9.0 * t1882 * t7499;
    let t33683 = t242 * t33264;
    let t33686 = t10002 * t7546;
    let t33687 = t242 * t33686;
    let t33690 = -t446 * t33651 / 3.0 - t446 * t33654 / 3.0 + t33658 + 2.0 / 3.0 * t446 * t33660 + t446 * t33665 / 3.0 + 4.0 / 3.0 * t446 * t33668 - 2.0 / 3.0 * t446 * t33673 - 2.0 * t446 * t33676 - t33680 + t33682 - 2.0 / 3.0 * t446 * t33683 + 2.0 / 3.0 * t446 * t33687;
    (t33680, t33682, t33683, t33686, t33687, t33690)
}
