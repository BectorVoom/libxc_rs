//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1106/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1106<F: Float>(t1083: F, t1899: F, t20716: F, t1088: F, t5870: F, t1100: F, t5490: F, t1898: F, t2743: F, t237: F, t5845: F, t307: F, t6000: F) -> (F, F, F, F, F, F, F) {
    let t21184 = t1899 * t1083;
    let t21191 = F::new(0.71233333333333333332e-1) * t20716;
    let t21203 = t1088 * t5870;
    let t21212 = t1100 * t5490;
    let t21221 = t2743 * t1898;
    let t21267 = t237 * t5845;
    let t21346 = t307 * t6000;
    (t21184, t21191, t21203, t21212, t21221, t21267, t21346)
}
