//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1208/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1208<F: Float>(t27279: F, t32478: F, t1032: F, t7997: F, t1955: F, t126250: F, t8477: F, t126210: F, t119894: F, t119913: F, t121891: F, t121896: F, t121897: F, t27300: F, t27322: F, t32434: F, t32464: F, t7079: F) -> (F, F, F) {
    let t127698 = t32478 * t27279;
    let t127703 = t7997 * t1032;
    let t127704 = t1955 * t127703;
    let t127707 = t8477 * t126250;
    let t127710 = F::new(0.263521689745817692e-2) * t126210;
    let t127711 = F::new(0.17347256376410398924e1) * t32434 * t27322 + t121891 + F::new(0.14456046980341999104e-1) * t127698 - F::new(0.66934509195437693771e-4) * t119894 + t121896 + t121897 - F::new(0.52041769129231196772e1) * t32434 * t27300 + F::new(0.8673628188205199462e0) * t127704 * t7079 - t119913 - F::new(0.11423947533020470523e1) * t127707 * t32464 + t127710;
    (t127703, t127704, t127711)
}
