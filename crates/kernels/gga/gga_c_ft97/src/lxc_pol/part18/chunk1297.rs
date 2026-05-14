//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1297/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1297<F: Float>(t1013: F, t397: F, t22591: F, t538: F, t2059: F, t26738: F, t554: F, t22632: F, t26650: F, t5829: F, t22642: F, t26634: F, t23839: F, t23832: F, t22638: F, t26612: F) -> (F, F, F, F, F, F, F, F) {
    let t104930 = t397 * t1013;
    let t104932 = t22591 * t104930 * t538;
    let t104941 = t22591 * t26738 * t2059;
    let t104945 = t22591 * t104930 * t554;
    let t104949 = t5829 * t22632 * t26650;
    let t104951 = t22642 * t26634;
    let t104953 = 0.1611184118048991131e0 * t23839 * t104951;
    let t104955 = 0.1611184118048991131e0 * t23832 * t104951;
    let t104956 = t22638 * t26634;
    let t104965 = t22642 * t26612;
    (t104932, t104941, t104945, t104949, t104953, t104955, t104956, t104965)
}
