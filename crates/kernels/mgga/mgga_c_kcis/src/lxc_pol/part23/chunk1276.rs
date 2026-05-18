//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1276/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1276<F: Float>(t28343: F, t94246: F, t7908: F, t27416: F, t27447: F, t27459: F, t28353: F, t28420: F, t28544: F, t37602: F, t491: F, t8159: F, t98174: F, t98823: F, t98825: F, t98830: F, t98835: F, t98845: F, t990: F) -> (F, F) {
    let t98847 = t94246 * t28343;
    let t98849 = F::new(0.46336805555555555556e-3) * t7908 * t98847;
    let t98850 = -t98823 + t98825 - F::new(0.24734586805555555555e-3) * t28544 * t27416 + F::new(0.69505208333333333333e-3) * t27447 * t8159 - F::new(0.3684876543209876543e-3) * t98830 + F::new(0.41703125000000000001e-2) * t7908 * t98174 + F::new(0.88437037037037037034e-2) * t98835 - F::new(0.37134344353515625e-4) * t37602 * t491 * t990 * t28353 + F::new(0.92673611111111111112e-3) * t27459 * t28420 - F::new(0.92673611111111111113e-3) * t98845 - t98849;
    (t98847, t98850)
}
