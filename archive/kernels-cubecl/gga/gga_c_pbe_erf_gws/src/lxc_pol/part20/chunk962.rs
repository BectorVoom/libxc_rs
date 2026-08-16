//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 962/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk962<F: Float>(t10778: F, t2559: F, t587: F, t10837: F, t10838: F, t10840: F, t10841: F, t10845: F, t10847: F, t10850: F, t10852: F, t10856: F, t5359: F, t7617: F, t7619: F, t7623: F, t7665: F, t7668: F, t7672: F) -> (F, F) {
    let t10857 = t2559 * t10778;
    let t10859 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t587 * t10857;
    let t10860 = -t10837 + t7617 + t7619 + t7623 - t10838 + t5359 - t10840 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10841 - t7665 - t7668 + t7672 + t10845 - t10847 + t10850 + t10852 - t10856 - t10859;
    (t10859, t10860)
}
