//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1402/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1402(t10439: f64, t1407: f64, t10418: f64, t1424: f64, t30897: f64, t30900: f64, t30902: f64, t30920: f64, t31558: f64, t31748: f64, t34567: f64, t34762: f64, t34766: f64, t34773: f64, t34774: f64, t34775: f64, t34776: f64, t34777: f64, t4372: f64, t4819: f64, t4820: f64, t6824: f64, t6963: f64, t6964: f64) -> f64 {
    let t34782 = t1407 * t10439;
    let t34783 = 0.85206502119823888168e-1_f64 * t34782;
    let t34784 = -0.14300195980740170668e1_f64 * t6963 * t6964 * t34567 - t34762 + t34766 - 0.79445533226334281486e-1_f64 * t4819 * t4820 * t31558 - 0.15889106645266856297e0_f64 * t6824 * t4820 * t31748 - t30897 - t30900 + t30902 - t34773 - t34774 + t30920 - t34775 - t34776 - 0.79445533226334281486e-1_f64 * t34777 * t1424 + 0.92686455430723328401e-1_f64 * t10418 * t4372 + t34783;
    t34784
}
