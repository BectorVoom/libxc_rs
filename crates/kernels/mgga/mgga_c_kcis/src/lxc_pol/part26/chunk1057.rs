//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1057/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1057<F: Float>(t29458: F, t29486: F, t1506: F, t2268: F, t7566: F, t2128: F, t8251: F, t7537: F, t12940: F, t18268: F, t23255: F, t27702: F, t28649: F, t29413: F, t29421: F, t4480: F, t6222: F, t7998: F, t8240: F) -> (F, F, F, F, F, F) {
    let t29487 = t29458 + t29486;
    let t29488 = t1506 * t29487;
    let t29489 = t2268 * t7566;
    let t29499 = t8251 * t2128;
    let t29502 = t2268 * t7537;
    let t29508 = -6.0 * t12940 * t29502 + 4.0 * t18268 * t8240 - 2.0 * t2128 * t28649 - t2268 * t23255 + 2.0 * t27702 * t7537 + 2.0 * t29489 * t4480 + 4.0 * t29499 * t4480 - 2.0 * t6222 * t8251 - t7566 * t7998 - t29413 - t29421;
    (t29487, t29488, t29489, t29499, t29502, t29508)
}
