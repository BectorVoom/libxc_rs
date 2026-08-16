//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1206/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1206(t11546: f64, t424: f64, t655: f64, t34873: f64, t34876: f64, t34878: f64, t34881: f64, t34884: f64, t34886: f64, t34889: f64, t34891: f64, t34894: f64, t34897: f64) -> f64 {
    let t34900 = t424 * t655 * t11546;
    let t34902 = -0.18115908419564701086e-6_f64 * t34873 - 0.22489692402754972536e-8_f64 * t34876 + 0.55984797807908795905e-7_f64 * t34878 + 0.82779637083844259127e-6_f64 * t34881 - 0.98332751566569010432e-8_f64 * t34884 + 0.14068827330203670243e-7_f64 * t34886 - 0.26194992237489957663e-8_f64 * t34889 - 0.20634280084298716356e-4_f64 * t34891 + 0.46574696198257144727e-9_f64 * t34894 + 0.1422820120100248667e-7_f64 * t34897 - 0.252977417353824213e-7_f64 * t34900;
    t34902
}
