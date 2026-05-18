//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1038/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1038<F: Float>(t2427: F, t6793: F, t224: F, t2344: F, t7205: F, t123619: F, t5009: F, t694: F, t109216: F, t3766: F, t6: F, t150319: F, t420: F) -> (F, F, F, F, F) {
    let t150843 = t2427 * t6793;
    let t150844 = t224 * t150843;
    let t150845 = t7205 * t2344;
    let t150846 = t150845 * t123619;
    let t150849 = t694 * t5009;
    let t150858 = t3766 * t109216 * t6;
    let t150864 = t150319 * t420;
    (t150844, t150846, t150849, t150858, t150864)
}
