//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1390/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1390<F: Float>(t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34127: F, t34132: F, t34125: F, t34135: F, t36824: F, t34142: F) -> (F, F) {
    let t36825 = F::cast_from(0.11272120794395814009e-6_f64) * t34108;
    let t36826 = F::cast_from(0.69504740211613770836e-3_f64) * t34111;
    let t36827 = F::cast_from(0.49163213094075520836e-7_f64) * t34114;
    let t36828 = F::cast_from(0.24581606547037760418e-8_f64) * t34117;
    let t36829 = F::cast_from(0.70341874126922921074e-8_f64) * t34119;
    let t36830 = F::cast_from(0.70341874126922921074e-8_f64) * t34121;
    let t36832 = F::cast_from(0.34179092986183952014e-5_f64) * t34127;
    let t36833 = F::cast_from(0.24581606547037760418e-8_f64) * t34132;
    let t36835 = t36824 + t36825 + t36826 - t36827 + t36828 - t36829 - t36830 + F::cast_from(0.95956020918421216158e-7_f64) * t34125 + t36832 - t36833 + F::cast_from(0.25301106770833333336e-5_f64) * t34135;
    let t36838 = F::cast_from(0.50680539737635041234e-3_f64) * t34142;
    (t36835, t36838)
}
