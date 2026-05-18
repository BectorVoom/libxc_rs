//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1026/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1026<F: Float>(t19862: F, t871: F, t296: F, t4129: F, t992: F, t2875: F, t2874: F, t1212: F, t18: F, t10447: F, t5414: F, t10749: F, t10773: F, t11593: F, t15467: F, t15471: F, t15491: F, t15500: F, t15502: F, t15532: F, t1901: F, t19811: F, t19816: F, t19819: F, t446: F) -> (F, F) {
    let t19863 = t871 * t19862;
    let t19864 = t296 * t19863;
    let t19867 = t992 * t4129;
    let t19868 = t2875 * t19867;
    let t19869 = t2874 * t19868;
    let t19872 = t18 * t1212;
    let t19873 = t2875 * t19872;
    let t19874 = t2874 * t19873;
    let t19877 = t10447 * t5414;
    let t19880 = -t10749 + F::new(2.0) / F::new(3.0) * t446 * t19811 - t15467 + t15471 + t15491 - F::new(4.0) / F::new(27.0) * t10773 - t15500 - t15502 - F::new(2.0) / F::new(9.0) * t1901 * t19816 - F::new(4.0) / F::new(9.0) * t1901 * t19819 - t15532 - t446 * t19864 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t1901 * t19869 - F::new(4.0) / F::new(9.0) * t11593 * t19874 + F::new(2.0) / F::new(9.0) * t1901 * t19877;
    (t19863, t19880)
}
