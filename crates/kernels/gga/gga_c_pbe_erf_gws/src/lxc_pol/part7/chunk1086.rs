//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1086/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1086<F: Float>(t20702: F, t20712: F, t20725: F, t20731: F, t20746: F, t20750: F, t20753: F, t20755: F, t20758: F, t20761: F, t20769: F, t20781: F, t20785: F, t20791: F, t20793: F, t20797: F, t20799: F, t20801: F, t20806: F, t20829: F, t20832: F, t20837: F) -> (F, F) {
    let t21696 = t20702 - t20712 - t20725 + t20731 + t20746 + t20750 + t20753 + t20755 + t20758 + t20761 - t20769;
    let t21697 = t20781 - t20785 + t20791 + t20793 + t20797 + t20799 + t20801 - t20806 + t20829 + t20832 - t20837;
    (t21696, t21697)
}
