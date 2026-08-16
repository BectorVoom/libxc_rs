//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1590/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1590<F: Float>(t1860: F, t23993: F, t6509: F, t7031: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22645: F, t225: F, t7192: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23995 = F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t1860 * t23993;
    let t23998 = t7031 * t6509;
    let t23999 = t1860 * t23998;
    let t24049 = F::cast_from(0.33643963411783659044e-4_f64) * t22819;
    let t24050 = F::cast_from(0.10541775202358879834e-2_f64) * t22825;
    let t24058 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t22858;
    let t24060 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t22863;
    let t24061 = F::cast_from(0.22608743412718618878e-1_f64) * t22867;
    let t24071 = F::cast_from(0.16449340668482264365e-1_f64) * t22645;
    let t24082 = t7192 * t225;
    (t23995, t23998, t23999, t24049, t24050, t24058, t24060, t24061, t24071, t24082)
}
