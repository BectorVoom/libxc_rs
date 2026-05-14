//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 964/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk964<F: Float>(t10248: F, t28755: F, t33847: F, t3746: F, t10683: F, t24980: F, t28776: F, t28741: F, t2862: F, t28816: F, t6318: F, t28735: F, t28736: F, t33868: F, t4162: F, t6317: F) -> (F, F, F, F, F, F) {
    let t152738 = t28755 * t10248 * t33847 * t3746;
    let t152742 = t24980 * t10683 * t33847 * t28776;
    let t152746 = t24980 * t10683 * t33847 * t28741;
    let t152750 = t24980 * t2862 * t6318 * t28816;
    let t152754 = t28735 * t2862 * t33847 * t28736;
    let t152758 = t6317 * t10683 * t33868 * t4162;
    (t152738, t152742, t152746, t152750, t152754, t152758)
}
