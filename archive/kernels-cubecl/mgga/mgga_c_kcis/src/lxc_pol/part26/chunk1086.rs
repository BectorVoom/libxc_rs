//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1086/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1086<F: Float>(t1655: F, t7671: F, t7889: F, t8130: F, t2272: F, t5407: F, t446: F, t1300: F, t8255: F, t7886: F, t1885: F, t8014: F) -> (F, F, F, F, F, F, F, F) {
    let t27741 = t1655 * t7671;
    let t28311 = t8130 * t7889;
    let t28313 = t5407 * t2272;
    let t28314 = t446 * t28313;
    let t28316 = t1300 * t8255;
    let t28317 = t446 * t28316;
    let t28320 = t8130 * t7886;
    let t28322 = t1885 * t8014;
    (t27741, t28311, t28313, t28314, t28316, t28317, t28320, t28322)
}
