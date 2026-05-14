//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 944/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk944<F: Float>(t1114: F, t11781: F, t2138: F, t11478: F, t2157: F, t337: F, t2121: F, t6645: F, t11746: F, t11750: F, t11754: F, t11758: F, t11762: F, t11766: F, t11768: F, t11770: F, t11772: F, t11775: F, t11780: F, t6637: F, t902: F) -> (F, F, F, F, F) {
    let t11782 = t1114 * t11781;
    let t11784 = t11782 * t2138 / 96.0;
    let t11785 = t11478 * t2157;
    let t11786 = t337 * t11785;
    let t11787 = t2121 * t11786;
    let t11789 = t6645 * t11787 / 48.0;
    let t11790 = t6637 * t11746 / 768.0 - t6637 * t11750 / 384.0 + t902 * t11754 / 1536.0 + t902 * t11758 / 1536.0 - t11762 + t11766 - t11768 + t11770 + t11772 - t11775 - t11780 - t11784 + t11789;
    (t11782, t11784, t11785, t11789, t11790)
}
