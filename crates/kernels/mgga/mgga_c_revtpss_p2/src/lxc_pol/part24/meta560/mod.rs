//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1683;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta560<F: Float>(t19049: F, t6223: F, t11465: F, t88008: F, t973: F, t981: F, t23696: F, t4719: F, t6227: F, t300: F, t88477: F, t23457: F, t88264: F, t964: F, t2986: F, t88351: F, t1642: F, t78704: F, t88445: F, t88448: F, t88451: F, t88481: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t88580, t88584, t88586, t88588, t88590, t88592) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1683::<F>(t19049, t6223, t11465, t88008, t973, t981, t23696, t4719, t6227, t300, t88477, t23457);
        let (t88596, t88600, t88602, t88603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1684::<F>(t88264, t964, t973, t981, t2986, t88351, t1642, t78704, t88445, t88448, t88451, t88481, t88580, t88584, t88586, t88588, t88590, t88592);
    (t88580, t88584, t88586, t88588, t88590, t88592, t88596, t88600, t88602, t88603)
}
