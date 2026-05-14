//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 762/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk762<F: Float>(t299: F, t5612: F, t1843: F, t655: F, t779: F, t2888: F, t2070: F, t771: F, t2006: F, t2074: F, t279: F, t2887: F, t303: F, t5591: F, t5592: F, t5595: F, t5597: F, t5604: F, t5607: F, t5609: F) -> (F, F, F, F) {
    let t5614 = 0.63517063878621832551e-4 * t299 * t5612;
    let t5616 = t779 * t1843 * t655;
    let t5617 = t2888 * t5616;
    let t5620 = t771 * t2070;
    let t5626 = t5591 - 77.0 / 162.0 * t5592 * t279 + 11.0 / 108.0 * t5595 + t5597 / 54.0 - 0.53100265402527852012e-1 * t5604 * t303 + 0.14481890564325777821e-1 * t5607 + 0.7622047665434619906e-3 * t5609 + t5614 + t2887 * t5617 / 16.0 + 0.91464571985215438873e-2 * t5620 + 0.68598428988911579154e-2 * t771 * t2074 - 0.20579528696673473747e-1 * t771 * t2006;
    (t5614, t5616, t5620, t5626)
}
