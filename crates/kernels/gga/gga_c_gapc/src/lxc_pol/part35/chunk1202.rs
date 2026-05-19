//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1202/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1202<F: Float>(t11546: F, t424: F, t655: F, t34873: F, t34876: F, t34878: F, t34881: F, t34884: F, t34886: F, t34889: F, t34891: F, t34894: F, t34897: F) -> F {
    let t34900 = t424 * t655 * t11546;
    let t34902 = -F::cast_from(0.18115908419564701086e-6_f64) * t34873 - F::cast_from(0.22489692402754972536e-8_f64) * t34876 + F::cast_from(0.55984797807908795905e-7_f64) * t34878 + F::cast_from(0.82779637083844259127e-6_f64) * t34881 - F::cast_from(0.98332751566569010432e-8_f64) * t34884 + F::cast_from(0.14068827330203670243e-7_f64) * t34886 - F::cast_from(0.26194992237489957663e-8_f64) * t34889 - F::cast_from(0.20634280084298716356e-4_f64) * t34891 + F::cast_from(0.46574696198257144727e-9_f64) * t34894 + F::cast_from(0.1422820120100248667e-7_f64) * t34897 - F::cast_from(0.252977417353824213e-7_f64) * t34900;
    t34902
}
