//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1216/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1216<F: Float>(t34092: F, t34100: F, t36812: F, t36813: F, t36814: F, t36815: F, t36816: F, t36817: F, t36818: F, t36820: F, t36821: F, t34125: F, t34135: F, t36824: F, t36825: F, t36826: F, t36827: F, t36828: F, t36829: F, t36830: F, t36832: F, t36833: F) -> (F, F) {
    let t38792 = -t36812 - t36813 - t36814 - t36815 - t36816 + t36817 + t36818 - 0.98380106748709416171e-8 * t34092 - t36820 + t36821 - 0.36231816839129402172e-6 * t34100;
    let t38795 = t36824 + t36825 + t36826 - t36827 + t36828 - t36829 - t36830 + 0.95956020918421216159e-7 * t34125 + t36832 - t36833 + 0.25301106770833333334e-5 * t34135;
    (t38792, t38795)
}
