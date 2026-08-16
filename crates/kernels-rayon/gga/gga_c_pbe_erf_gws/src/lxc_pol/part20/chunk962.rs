//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 962/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk962(t10778: f64, t2559: f64, t587: f64, t10837: f64, t10838: f64, t10840: f64, t10841: f64, t10845: f64, t10847: f64, t10850: f64, t10852: f64, t10856: f64, t5359: f64, t7617: f64, t7619: f64, t7623: f64, t7665: f64, t7668: f64, t7672: f64) -> (f64, f64) {
    let t10857 = t2559 * t10778;
    let t10859 = 8.0_f64 / 9.0_f64 * t587 * t10857;
    let t10860 = -t10837 + t7617 + t7619 + t7623 - t10838 + t5359 - t10840 + 2.0_f64 / 9.0_f64 * t10841 - t7665 - t7668 + t7672 + t10845 - t10847 + t10850 + t10852 - t10856 - t10859;
    (t10859, t10860)
}
