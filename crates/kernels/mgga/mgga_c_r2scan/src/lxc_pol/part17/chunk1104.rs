//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1104/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1104<F: Float>(t146: F, t2206: F, t2832: F, t37848: F, t37851: F, t10810: F, t1592: F, t8156: F, t10743: F, t2699: F, t37890: F, t924: F) -> (F, F, F, F, F, F) {
    let t39745 = t146 * t2206 * t2832;
    let t39752 = F::new(0.84755945902752848174e0) * t37848;
    let t39753 = F::new(0.25426783770825854452e1) * t37851;
    let t39762 = t1592 * t10810 * t8156;
    let t39770 = t10743 * t2699;
    let t39772 = t37890 * t924;
    (t39745, t39752, t39753, t39762, t39770, t39772)
}
