//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1381/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1381<F: Float>(t32965: F, t415: F, t8940: F, t1799: F, t34093: F, t6668: F, t5054: F, t6676: F, t32903: F, t8485: F, t22971: F, t6713: F, t9679: F, t22316: F, t33017: F, t22321: F) -> (F, F, F, F, F, F, F) {
    let t121796 = t415 * t32965 * t8940;
    let t121800 = t1799 * t34093 * t6668;
    let t121803 = t5054 * t34093 * t6676;
    let t121806 = t1799 * t32903 * t8485;
    let t121809 = t6713 * t9679 * t22971;
    let t121812 = t1799 * t33017 * t22316;
    let t121815 = t1799 * t9679 * t22321;
    (t121796, t121800, t121803, t121806, t121809, t121812, t121815)
}
