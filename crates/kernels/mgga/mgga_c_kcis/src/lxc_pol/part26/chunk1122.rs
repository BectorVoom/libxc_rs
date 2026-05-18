//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1122/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1122<F: Float>(t28613: F, t28643: F, t1506: F, t1628: F, t8236: F, t1636: F, t17710: F, t18268: F, t2268: F, t27702: F, t28562: F, t28563: F, t28564: F, t28566: F, t28567: F, t28569: F, t28572: F, t28575: F, t28578: F, t28582: F, t6225: F, t6256: F, t7998: F, t8001: F) -> (F, F, F, F) {
    let t28644 = t28613 + t28643;
    let t28645 = t1506 * t28644;
    let t28649 = t8236 * t1628;
    let t28654 = -t1636 * t28649 - t17710 * t2268 + F::new(2.0) * t18268 * t8001 + F::new(2.0) * t27702 * t6225 - t6256 * t7998 - t28562 + t28563 + t28564 - t28566 + t28567 - t28569 + t28572 - t28575 - t28578 - t28582;
    (t28644, t28645, t28649, t28654)
}
