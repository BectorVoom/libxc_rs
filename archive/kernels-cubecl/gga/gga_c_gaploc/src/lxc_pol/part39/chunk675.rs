//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 675/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk675<F: Float>(t10900: F, t10876: F, t10878: F, t10881: F, t10885: F, t10888: F, t10891: F, t10893: F, t10899: F, t2028: F, t9836: F, t9838: F, t9846: F, t9849: F, t9853: F, t9892: F) -> (F, F) {
    let t10901 = F::cast_from(0.14896037479937677779e-1_f64) * t10900;
    let t10902 = -t9836 + t9838 - t9846 - t9849 + t9853 - t10876 + t10878 + t10881 - t10885 + t10888 - t10891 - F::cast_from(0.39722766613167140743e-1_f64) * t10893 * t2028 + t10899 - t9892 + t10901;
    (t10901, t10902)
}
