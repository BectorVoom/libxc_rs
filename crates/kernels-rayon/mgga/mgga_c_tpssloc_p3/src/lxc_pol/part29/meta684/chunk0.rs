//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2326/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2326(t24682: f64, t460: f64, t95484: f64, t27634: f64, t3030: f64, t86259: f64, t24740: f64, t5064: f64, t15640: f64, t24729: f64, t14726: f64, t15394: f64, t2121: f64, t2132: f64, t2133: f64, t24706: f64, t27639: f64, t27645: f64, t27674: f64, t27704: f64, t3552: f64, t3557: f64, t3580: f64, t4928: f64, t7321: f64, t7331: f64, t86365: f64, t86368: f64) -> f64 {
    let t95678 = t24682 * t95484 * t460;
    let t95682 = t27634 * t86259 * t3030;
    let t95687 = t5064 * t24740;
    let t95702 = t24729 * t15640 / 576.0_f64;
    let t95703 = -0.20186378047070195428e-3_f64 * t2132 * t2133 * t4928 * t7321 - 0.20186378047070195428e-3_f64 * t95678 * t7331 - 0.40372756094140390856e-3_f64 * t95682 * t27639 + 0.20186378047070195428e-3_f64 * t95682 * t27645 - t95687 * t3580 / 1152.0_f64 - 0.10093189023535097714e-3_f64 * t27704 * t24706 - 7.0_f64 / 648.0_f64 * t2121 * t15394 * t14726 + t86365 / 648.0_f64 - 0.10093189023535097714e-3_f64 * t86368 + t27674 * t3552 / 108.0_f64 + t27674 * t3557 / 54.0_f64 + t95702;
    t95703
}
