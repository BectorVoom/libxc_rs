//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 981/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk981<F: Float>(t14097: F, t5176: F, t5175: F, t14374: F, t359: F, t376: F, t1170: F, t3474: F, t5053: F, t1809: F, t3448: F, t10745: F, t5099: F) -> (F, F, F, F, F, F) {
    let t14812 = t5176 * t14097;
    let t14813 = t5175 * t14812;
    let t14815 = t359 * t14374;
    let t14816 = t376 * t14815;
    let t14817 = t1170 * t14816;
    let t14819 = t3474 * t5053;
    let t14821 = t1809 * t3448;
    let t14823 = t10745 * t5099;
    (t14812, t14813, t14817, t14819, t14821, t14823)
}
