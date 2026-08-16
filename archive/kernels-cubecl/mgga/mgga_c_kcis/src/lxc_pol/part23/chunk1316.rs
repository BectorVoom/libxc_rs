//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1316/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1316<F: Float>(t97877: F, t97961: F, t97976: F, t97977: F, t97979: F, t97984: F, t97989: F, t97990: F, t97993: F, t97996: F, t98956: F, t98959: F, t98963: F, t99713: F, t99716: F, t99717: F, t99723: F, t99726: F, t99728: F) -> F {
    let t99738 = -t97877 - t97961 - t97976 - t97977 - t97979 - t97984 + t97989 - t97990 + t97993 + t97996 + t98956 + t98959 - t98963 - t99713 + t99716 - t99717 - t99723 - t99726 + t99728;
    t99738
}
