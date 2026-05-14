//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1018/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1018<F: Float>(t3031: F, t6423: F, t4764: F, t13864: F, t4690: F, t13872: F, t4723: F, t18866: F, t18868: F, t18870: F, t18872: F, t18874: F, t18947: F, t18949: F, t18965: F, t18970: F, t18973: F, t18976: F, t18980: F, t18983: F, t18987: F, t45: F, t960: F) -> (F, F, F, F) {
    let t18988 = t3031 * t6423;
    let t18989 = t18988 * t4764;
    let t18993 = 4.0 * t13864 * t4690;
    let t18995 = 0.32163648644302209644e2 * t13872 * t4723;
    let t18996 = t18866 + t18868 + t18870 - t18872 + t18874 + t18947 + t18949 + 0.19751789702565206229e-1 * t45 * t18965 - t18970 - t18973 - t18976 + t18980 + t18983 + t18987 - 0.17315755899375863299e2 * t960 * t18989 - t18993 + t18995;
    (t18989, t18993, t18995, t18996)
}
