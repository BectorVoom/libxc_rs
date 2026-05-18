//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1171/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1171<F: Float>(t120991: F, t121019: F, t5676: F, t121018: F, t5674: F, t94396: F, t1404: F, t32276: F, t33963: F, t32206: F, t32211: F, t5673: F, t5774: F) -> (F, F, F, F) {
    let t125749 = t120991 * t121019 * t5676;
    let t125753 = t121018 * t121019 * t5674 * t94396;
    let t125767 = t32276 * t1404 * t33963;
    let t125771 = t32206 * t5673 * t32211 * t5774;
    (t125749, t125753, t125767, t125771)
}
