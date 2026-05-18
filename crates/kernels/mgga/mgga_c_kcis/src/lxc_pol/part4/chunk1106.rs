//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1106/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1106<F: Float>(t13717: F, t13742: F, t13772: F, t13775: F, t13777: F, t13881: F, t13886: F, t13888: F, t13892: F, t13910: F, t13912: F, t13915: F, t13918: F, t13921: F, t13924: F, t13927: F, t13931: F, t13934: F, t13951: F, t9681: F, t9683: F, t9691: F) -> F {
    let t13953 = F::new(0.142419375e1) * t13772 - F::new(0.76790625e-1) * t13881 - F::new(0.1898925e1) * t13775 - F::new(0.9494625e0) * t13777 + F::new(0.3071625e0) * t13886 + F::new(0.15358125e0) * t13888 - F::new(0.16431333333333333333e0) * t13892 + F::new(0.99655555555555555557e-1) * t9681 + F::new(0.66437037037037037038e-1) * t9683 - F::new(0.26574814814814814816e0) * t9691 + t13910 + F::new(0.36514074074074074074e-1) * t13912 - F::new(0.27385555555555555556e-1) * t13915 - F::new(0.36514074074074074075e-1) * t13918 - F::new(0.10954222222222222222e0) * t13921 + F::new(0.16431333333333333333e0) * t13924 + F::new(0.65725333333333333332e0) * t13927 + F::new(0.21924222222222222222e1) * t13717 + F::new(0.16431333333333333333e0) * t13931 - F::new(0.49293999999999999999e0) * t13934 - F::new(0.59793333333333333334e0) * t13742 + t13951;
    t13953
}
