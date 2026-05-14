//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1046/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1046<F: Float>(t11546: F, t424: F, t655: F, t34873: F, t34876: F, t34878: F, t34881: F, t34884: F, t34886: F, t34889: F, t34891: F, t34894: F, t34897: F, t1266: F, t3696: F, t3703: F) -> (F, F) {
    let t34900 = t424 * t655 * t11546;
    let t34902 = -0.18115908419564701086e-6 * t34873 - 0.22489692402754972536e-8 * t34876 + 0.55984797807908795905e-7 * t34878 + 0.82779637083844259127e-6 * t34881 - 0.98332751566569010432e-8 * t34884 + 0.14068827330203670243e-7 * t34886 - 0.26194992237489957663e-8 * t34889 - 0.20634280084298716356e-4 * t34891 + 0.46574696198257144727e-9 * t34894 + 0.1422820120100248667e-7 * t34897 - 0.252977417353824213e-7 * t34900;
    let t34905 = t1266 * t3696 * t3703;
    (t34902, t34905)
}
