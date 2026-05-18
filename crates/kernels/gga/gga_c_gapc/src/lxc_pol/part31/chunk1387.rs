//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1387/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1387<F: Float>(t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34127: F, t34132: F, t34142: F, t34144: F, t34146: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36824 = F::new(0.4637672555408563478e-4) * t34104;
    let t36825 = F::new(0.11272120794395814009e-6) * t34108;
    let t36826 = F::new(0.69504740211613770836e-3) * t34111;
    let t36827 = F::new(0.49163213094075520836e-7) * t34114;
    let t36828 = F::new(0.24581606547037760418e-8) * t34117;
    let t36829 = F::new(0.70341874126922921074e-8) * t34119;
    let t36830 = F::new(0.70341874126922921074e-8) * t34121;
    let t36832 = F::new(0.34179092986183952014e-5) * t34127;
    let t36833 = F::new(0.24581606547037760418e-8) * t34132;
    let t36838 = F::new(0.50680539737635041234e-3) * t34142;
    let t36839 = F::new(0.20240885416666666668e-4) * t34144;
    let t36840 = F::new(0.20240885416666666668e-3) * t34146;
    (t36824, t36825, t36826, t36827, t36828, t36829, t36830, t36832, t36833, t36838, t36839, t36840)
}
