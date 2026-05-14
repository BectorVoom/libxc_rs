//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1236/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1236<F: Float>(t10513: F, t20441: F, t6914: F, t10532: F, t4529: F, t579: F, t30903: F, t30907: F, t30923: F, t30927: F, t10417: F, t1397: F, t10439: F, t1407: F, t10418: F, t1424: F, t30897: F, t30900: F, t30902: F, t30920: F, t31558: F, t31748: F, t34567: F, t4372: F, t4819: F, t4820: F, t6824: F, t6963: F, t6964: F) -> (F,) {
    let t34762 = 0.1656414401209376386e3 * t6914 * t20441 * t10513;
    let t34766 = 0.73618417831527839379e2 * t10532 * t579 * t4529 * t10513;
    let t34773 = 0.63904876589867916128e-1 * t30903;
    let t34774 = 0.95857314884801874192e-1 * t30907;
    let t34775 = 0.31952438294933958064e-1 * t30923;
    let t34776 = 0.12780975317973583226e0 * t30927;
    let t34777 = t1397 * t10417;
    let t34782 = t1407 * t10439;
    let t34783 = 0.85206502119823888168e-1 * t34782;
    let t34784 = -0.14300195980740170668e1 * t6963 * t6964 * t34567 - t34762 + t34766 - 0.79445533226334281486e-1 * t4819 * t4820 * t31558 - 0.15889106645266856297e0 * t6824 * t4820 * t31748 - t30897 - t30900 + t30902 - t34773 - t34774 + t30920 - t34775 - t34776 - 0.79445533226334281486e-1 * t34777 * t1424 + 0.92686455430723328401e-1 * t10418 * t4372 + t34783;
    (t34784,)
}
