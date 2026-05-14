//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1445/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1445<F: Float>(t33332: F, t33334: F, t33336: F, t109134: F, t109135: F, t109136: F, t109141: F, t109144: F, t113271: F, t113272: F, t113273: F, t113288: F, t113308: F, t116010: F, t116023: F, t31824: F, t31868: F, t31870: F, t31872: F, t31874: F, t31879: F, t31882: F, t31888: F, t8: F) -> (F,) {
    let t116027 = t33332 / 8.0;
    let t116028 = t33334 / 8.0;
    let t116029 = t33336 / 8.0;
    let t116030 = t113271 - t113272 + t113273 - t31824 + t8 * (t113288 + t113308 + t116010 + t116023) - t116027 - t109134 + t109135 - t109136 + t31868 + t31870 + t116028 + t31872 + t31874 + t109141 - t31879 + t116029 - t31882 + t109144 - t31888;
    (t116030,)
}
