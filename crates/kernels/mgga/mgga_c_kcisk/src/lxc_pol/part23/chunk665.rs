//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 665/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk665<F: Float>(t2206: F, t970: F, t457: F, t5671: F, t1383: F, t5676: F, t1186: F, t1398: F, t1375: F, t158: F, t165: F, t173: F, t3819: F, t3852: F, t3858: F, t3864: F, t3873: F, t3881: F, t3891: F, t5626: F) -> (F, F, F, F, F, F, F) {
    let t5836 = t970 * t2206;
    let t5845 = t457 * t5671;
    let t5848 = t1383 * t5676;
    let t5851 = t1186 * t5671;
    let t5854 = t1398 * t5676;
    let t5857 = t1375 * t5671;
    let t5860 = t1375 * t5676;
    let t5863 = -0.117630625e-4 * t5836 - 0.23911438650126355246e-1 * t3819 * t5626 + 0.15538616723388920628e-3 * t3891 * t5626 - 0.13208333333333333333e-2 * t3881 - 0.117630625e-4 * t3864 + 0.4684e-2 * t3873 + 0.1171e-2 * t158 * t5845 - 0.1585e-2 * t165 * t5848 - 0.52833333333333333333e-3 * t165 * t5851 - 0.10082625e-4 * t173 * t5854 - 0.672175e-5 * t173 * t5857 + 0.7026e-2 * t158 * t5860 - t3852 + t3858;
    (t5845, t5848, t5851, t5854, t5857, t5860, t5863)
}
