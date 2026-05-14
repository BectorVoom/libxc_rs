//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1195/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1195<F: Float>(t31179: F, t8392: F, t10007: F, t109960: F, t109962: F, t109968: F, t109989: F, t110478: F, t110539: F, t17790: F, t17794: F, t18201: F, t18402: F, t18460: F, t18502: F, t18686: F, t1901: F, t24599: F, t24789: F, t2574: F, t31155: F, t3891: F, t446: F, t4965: F, t5181: F, t6079: F, t6154: F, t684: F, t729: F, t97522: F, t97777: F, t97793: F) -> (F,) {
    let t122098 = t8392 * t31179;
    let t122116 = 2.0 / 27.0 * t1901 * t97522 * t18460 + 2.0 / 3.0 * t446 * t2574 * t5181 * t6079 - 2.0 / 9.0 * t1901 * t97793 * t18686 + 2.0 / 27.0 * t1901 * t3891 * t24599 * t4965 + t109960 + t109962 + 2.0 / 3.0 * t446 * t729 * t6154 * t18201 - 2.0 / 81.0 * t122098 - t109968 - t109989 - 2.0 / 9.0 * t1901 * t24789 * t18502 - 2.0 / 9.0 * t1901 * t97777 * t18402 - 4.0 / 9.0 * t1901 * t110478 * t17790 + 4.0 / 27.0 * t1901 * t110539 * t17794 - 2.0 / 9.0 * t1901 * t10007 * t31155 * t684;
    (t122116,)
}
