//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1182/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1182<F: Float>(t240: F, t33617: F, t33619: F, t33620: F, t33622: F, t33623: F, t33624: F, t33626: F, t33627: F, t33629: F, t33632: F, t33635: F, t33638: F, t33639: F, t33642: F, t33703: F, t33747: F, t33977: F) -> (F,) {
    let t33980 = t33617 - t33619 - t33620 + t33622 - t33623 - t33624 + t33626 - t33627 + t33629 - t33632 + t33635 + t33638 - t33639 + t33642 - t33703 + t240 * (t33747 + t33977);
    (t33980,)
}
