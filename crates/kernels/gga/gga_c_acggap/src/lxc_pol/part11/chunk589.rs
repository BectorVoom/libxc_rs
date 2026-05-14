//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 589/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk589<F: Float>(t1055: F, t4822: F, t345: F, t3579: F, t4798: F, t4800: F, t4802: F, t4804: F, t4809: F, t4812: F, t4814: F, t4817: F, t4820: F, t3132: F, t4353: F, t3112: F, t3118: F, t3122: F, t3128: F, t3130: F, t3144: F, t3146: F, t3580: F, t3588: F, t3592: F) -> (F, F, F, F) {
    let t4823 = t1055 * t4822;
    let t4824 = t345 * t4823;
    let t4826 = t4798 + t4800 - 0.36675e0 * t4802 + 0.2445e0 * t4804 - t4809 - 0.12225e0 * t4812 - 0.1141e1 * t4814 - t4817 + 0.1467e1 * t4820 + 0.7335e0 * t4824 + t3579;
    let t4833 = t3132 * t4353;
    let t4834 = t345 * t4833;
    let t4837 = -t3580 + 0.489e0 * t3112 + 0.12225e0 * t3118 - 0.61125e-1 * t3122 - 0.2445e0 * t3128 - 0.978e0 * t3130 - t3588 - 0.2282e1 * t3144 - 0.22005e1 * t4834 + 0.489e0 * t3146 + t3592;
    (t4824, t4826, t4834, t4837)
}
