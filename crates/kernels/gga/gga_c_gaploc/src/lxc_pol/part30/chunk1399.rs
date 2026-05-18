//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1399/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1399<F: Float>(t10439: F, t1407: F, t10418: F, t1424: F, t30897: F, t30900: F, t30902: F, t30920: F, t31558: F, t31748: F, t34567: F, t34762: F, t34766: F, t34773: F, t34774: F, t34775: F, t34776: F, t34777: F, t4372: F, t4819: F, t4820: F, t6824: F, t6963: F, t6964: F) -> F {
    let t34782 = t1407 * t10439;
    let t34783 = F::new(0.85206502119823888168e-1) * t34782;
    let t34784 = -F::new(0.14300195980740170668e1) * t6963 * t6964 * t34567 - t34762 + t34766 - F::new(0.79445533226334281486e-1) * t4819 * t4820 * t31558 - F::new(0.15889106645266856297e0) * t6824 * t4820 * t31748 - t30897 - t30900 + t30902 - t34773 - t34774 + t30920 - t34775 - t34776 - F::new(0.79445533226334281486e-1) * t34777 * t1424 + F::new(0.92686455430723328401e-1) * t10418 * t4372 + t34783;
    t34784
}
