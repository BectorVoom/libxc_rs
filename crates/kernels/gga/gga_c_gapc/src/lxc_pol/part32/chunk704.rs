//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 704/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk704<F: Float>(t4979: F, t8907: F, t1631: F, t190: F, t3707: F, t1743: F, t3113: F, t3112: F, t3117: F, t3123: F, t8798: F, t611: F, t8769: F, t5409: F, t8878: F, t8881: F, t8886: F, t8889: F, t8891: F, t8898: F, t8901: F, t8904: F) -> (F, F, F, F, F) {
    let t8908 = t8907 * t4979;
    let t8910 = t1631 * t190;
    let t8911 = t8910 * t3707;
    let t8912 = t1743 * t8911;
    let t8913 = t8912 * t4979;
    let t8915 = t3113 * t3707;
    let t8916 = t3112 * t8915;
    let t8917 = t8916 * t3117;
    let t8919 = t8798 * t3123;
    let t8921 = t611 * t8769;
    let t8922 = t8921 * t5409;
    let t8924 = -0.84412963981222021454e-7 * t8878 - 0.10005749997240850277e-7 * t8881 - 0.49240895655712845848e-7 * t8886 + 0.12380568050579229813e-5 * t8889 - 0.12163329537032409896e-2 * t8891 - 0.37545833188964626383e-6 * t8898 + 0.2318836277704281739e-4 * t8901 + 0.20241536458333333334e-4 * t8904 - 0.89947526170248609072e-8 * t8908 + 0.1199450261480265202e-7 * t8913 - 0.45020247456651744774e-7 * t8917 - 0.44082325634638166759e-6 * t8919 + 0.25745714186718600948e-5 * t8922;
    (t8910, t8911, t8915, t8916, t8924)
}
