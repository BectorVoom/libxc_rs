//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 927/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk927<F: Float>(t10892: F, t787: F, t2465: F, t2949: F, t2464: F, t825: F, t8516: F, t959: F, t10876: F, t10878: F, t10881: F, t10885: F, t10888: F, t10891: F, t2028: F, t9836: F, t9838: F, t9846: F, t9849: F, t9853: F, t9892: F) -> (F, F, F, F) {
    let t10893 = t787 * t10892;
    let t10896 = t2465 * t2949;
    let t10897 = t2464 * t10896;
    let t10898 = t825 * t10897;
    let t10899 = 0.42603251059911944084e-1 * t10898;
    let t10900 = t8516 * t959;
    let t10901 = 0.14896037479937677779e-1 * t10900;
    let t10902 = -t9836 + t9838 - t9846 - t9849 + t9853 - t10876 + t10878 + t10881 - t10885 + t10888 - t10891 - 0.39722766613167140743e-1 * t10893 * t2028 + t10899 - t9892 + t10901;
    (t10893, t10896, t10897, t10902)
}
