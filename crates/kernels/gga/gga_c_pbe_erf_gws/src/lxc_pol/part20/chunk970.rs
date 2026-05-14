//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 970/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk970<F: Float>(t11810: F, t11812: F, t11816: F, t11818: F, t11833: F, t11838: F, t11844: F, t11862: F, t11863: F, t11867: F, t11870: F, t11874: F, t11876: F, t11880: F, t11885: F, t11888: F, t11893: F, t11907: F, t11911: F, t11913: F, t11923: F, t11927: F, t9041: F, t9086: F, t9096: F) -> (F, F) {
    let t12156 = -t11810 + t11812 - t11816 - t11818 + t11833 + t11838 + t11844 + t11862 - t11863 - t11867 + t11870 - t11874;
    let t12157 = t11876 - t9041 + t11880 + t11885 - t11888 + t11893 + t9086 - t9096 - t11907 + t11911 + t11913 - t11923 + t11927;
    (t12156, t12157)
}
