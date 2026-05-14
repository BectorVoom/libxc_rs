//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1090/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1090<F: Float>(t22632: F, t26650: F, t5829: F, t22642: F, t26634: F, t23839: F, t23832: F, t26612: F, t23825: F, t23842: F, t12374: F, t5828: F, t100806: F, t5838: F, t100495: F, t23732: F, t26715: F) -> (F, F, F, F, F, F, F, F, F) {
    let t104949 = t5829 * t22632 * t26650;
    let t104951 = t22642 * t26634;
    let t104953 = 0.1611184118048991131e0 * t23839 * t104951;
    let t104955 = 0.1611184118048991131e0 * t23832 * t104951;
    let t104965 = t22642 * t26612;
    let t104967 = 0.1611184118048991131e0 * t23825 * t104965;
    let t104969 = 0.1611184118048991131e0 * t23842 * t104965;
    let t105007 = t12374 * t5828;
    let t105038 = t5838 * t100806;
    let t105044 = t5838 * t100495;
    let t105056 = 0.13335600218518518519e0 * t23732 * t22632 * t26715;
    (t104949, t104953, t104955, t104967, t104969, t105007, t105038, t105044, t105056)
}
