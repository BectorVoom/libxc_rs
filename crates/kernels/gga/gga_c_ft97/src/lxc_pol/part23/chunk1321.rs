//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1321/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1321<F: Float>(t10703: F, t112848: F, t112853: F, t112865: F, t112866: F, t112975: F, t1212: F, t15254: F, t15294: F, t15312: F, t18712: F, t1901: F, t19338: F, t19526: F, t19564: F, t24873: F, t24886: F, t28859: F, t28924: F, t29202: F, t29207: F, t4167: F, t4181: F, t446: F, t5414: F, t56522: F, t7032: F, t7105: F, t840: F, t871: F, t98753: F, t99034: F) -> (F,) {
    let t126069 = -4.0 / 27.0 * t98753 - t112848 - t112853 + t112865 - 8.0 / 27.0 * t112866 + 4.0 * t1901 * t112975 * t7105 * t4181 + 2.0 / 9.0 * t1901 * t56522 * t7032 + 2.0 / 3.0 * t446 * t840 * t28859 * t4167 + 2.0 / 9.0 * t1901 * t99034 * t5414 + 2.0 / 9.0 * t1901 * t24886 * t19564 + 2.0 / 3.0 * t446 * t840 * t871 * t28924 * t1212 - t1901 * t10703 * t24873 * t19338 / 9.0 - 2.0 / 9.0 * t1901 * t15312 * t24873 * t19526 - 2.0 / 9.0 * t1901 * t15254 * t29202 * t18712 + 2.0 / 27.0 * t1901 * t15294 * t29207 * t18712;
    (t126069,)
}
