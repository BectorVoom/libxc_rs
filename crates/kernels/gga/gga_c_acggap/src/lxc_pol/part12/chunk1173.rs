//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1173/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1173<F: Float>(t34488: F, t34500: F, t34506: F, t34508: F, t34510: F, t34512: F, t30601: F, t30605: F, t30607: F, t30611: F, t30613: F, t32517: F, t34482: F, t34492: F, t34497: F, t34502: F, t34504: F, t34516: F) -> F {
    let t37121 = F::new(0.916875e-1) * t34488;
    let t37126 = F::new(0.68598428988911579156e-2) * t34500;
    let t37129 = F::new(0.34299214494455789578e-2) * t34506;
    let t37130 = F::new(0.31448092289604152068e-2) * t34508;
    let t37131 = F::new(0.13208198761633743869e-1) * t34510;
    let t37132 = F::new(0.32012600194825403606e-1) * t34512;
    let t37134 = -F::new(0.51448821741683684367e-2) * t34482 + t32517 - t30601 / F::new(32.0) - t30605 / F::new(96.0) + F::new(0.28015625e-1) * t30607 + t37121 - F::new(0.31448092289604152069e-3) * t34492 - F::new(0.51448821741683684368e-2) * t30611 + F::new(0.12579236915841660828e-2) * t34497 - F::new(0.51448821741683684368e-2) * t30613 + t37126 - F::new(0.34299214494455789578e-2) * t34502 - F::new(0.17149607247227894789e-2) * t34504 + t37129 - t37130 + t37131 - t37132 + F::new(0.41930789719472202759e-2) * t34516;
    t37134
}
