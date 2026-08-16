//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2458/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2458<F: Float>(t68441: F, t68706: F, t68708: F, t68710: F, t68715: F, t68717: F, t68760: F, t68762: F, t68764: F, t68767: F, t68769: F, t68771: F, t68773: F, t68775: F, t68883: F, t68885: F, t68887: F, t68891: F, t68894: F, t68896: F) -> F {
    let t69958 = -t68441 - t68706 + t68708 - t68710 - t68715 - t68717 + t68760 + t68762 + t68764 + t68767 + t68769 + t68771 - t68773 + t68775 + t68883 + t68885 - t68887 + t68891 - t68894 - t68896;
    t69958
}
