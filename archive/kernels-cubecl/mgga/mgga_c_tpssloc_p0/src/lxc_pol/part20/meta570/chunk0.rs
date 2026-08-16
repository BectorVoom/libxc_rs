//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2132/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2132<F: Float>(t2986: F, t2990: F, t42771: F, t10346: F, t2987: F, t10190: F, t10245: F, t10250: F, t13779: F, t10255: F, t2989: F, t9258: F) -> (F, F, F, F, F, F) {
    let t42773 = t2986 * t42771 * t2990;
    let t42775 = t2987 * t10346;
    let t42785 = t2986 * t10190 * t10245;
    let t42788 = t2986 * t13779 * t10250;
    let t42794 = t2986 * t10190 * t10255;
    let t42799 = t2989 * t9258;
    (t42773, t42775, t42785, t42788, t42794, t42799)
}
