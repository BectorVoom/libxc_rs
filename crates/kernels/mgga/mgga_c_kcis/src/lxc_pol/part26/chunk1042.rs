//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1042/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1042<F: Float>(t187: F, t28557: F, t28559: F, t28560: F, t28562: F, t28563: F, t28564: F, t28566: F, t28567: F, t28569: F, t28572: F, t28575: F, t28578: F, t28579: F, t28582: F, t28645: F, t28654: F, t28873: F) -> (F,) {
    let t28876 = t28557 - t28559 - t28560 + t28562 - t28563 - t28564 + t28566 - t28567 + t28569 - t28572 + t28575 + t28578 - t28579 + t28582 - t28645 + t187 * (t28654 + t28873);
    (t28876,)
}
